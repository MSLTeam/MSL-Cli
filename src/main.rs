// 引用的模块部分
use chrono::Local;
use clap::{Arg, ArgAction, Command, Parser, Subcommand};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{
    env,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    process::exit,
};
use whoami;
use keyring::Entry;

// 常量定义数据结构和请求函数
const CODE_PERMISSION: i32 = 2;
const CODE_IO: i32         = 3;
const CODE_DISAGREE: i32   = 4;

#[derive(Deserialize)]
struct ApiResponse<T> {
    code:    u32,
    #[serde(rename = "msg")]
    message: String,
    data:    T,
}

#[derive(Deserialize)]
struct LoginRespData {
    username: String,
    token:    String,
}

#[derive(Parser)]
#[command(name = "msl-cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Login {
        /// 登录用户名
        #[arg(long)]
        username: String,

        /// 登录密码
        #[arg(long)]
        password: String,

        /// 保存token到系统钥匙串
        #[arg(short, long)]
        save: bool,
    },
}

const BASE_URL: &str = "https://user.mslmc.net";

//定义登陆返回数据和请求函数
fn login_user(
    email: &str,
    password: &str,
    twofa: Option<&str>,
) -> Result<LoginRespData, Box<dyn Error>> {
    // 构造 form 表单
    let mut form = vec![
        ("email",    email),
        ("password", password),
    ];
    if let Some(code) = twofa {
        form.push(("twoFactorAuthKey", code));
    }

    let resp: ApiResponse<LoginRespData> = Client::new()
        .post(format!("{}/api/user/login", BASE_URL))
        .form(&form)
        .send()?
        .error_for_status()?
        .json()?;

    if resp.code == 200 {
        Ok(resp.data)
    } else {
        Err(format!("登录失败: {}", resp.message).into())
    }
}

// help与version命令
fn main() {
    let matches = Command::new("msl-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Yuyi-Oak <awanye@qq.com>")
        .about("A blazing-fast, cross-platform CLI tool — 快速、跨平台的命令行工具")
        .long_about(
            "🌟 MSL-Cli 是一个跨平台命令行工具，灵感来自 MSL 项目，使用 Rust 编写，专注于性能与可扩展性。\n\
             支持 macOS 和 Linux 系统。\n\n\
             🔧 主要特性：\n\
             - 零依赖运行（纯二进制）\n\
             - 高性能、安全性强（Rust 加持）\n\
             - 易于扩展的 CLI 架构\n\n\
             🚀 欢迎 star、fork 与贡献！\n\n\
             ———\n\
             🌟 MSL-Cli is a blazing-fast, cross-platform command-line tool inspired by MSL and written in Rust.\n\
             Designed for macOS and Linux, it aims to be minimal, efficient, and extensible.\n\n\
             🔧 Features:\n\
             - Zero dependencies (single binary)\n\
             - Safe and fast (built in Rust)\n\
             - Modular CLI design\n\n\
             🚀 With more subcommands on the roadmap. Contributions welcome!"
        )
        // Disable the default version flag
        // 禁用原来的 -V
        .disable_version_flag(true)
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("输出版本信息 | Print version information")
                .action(ArgAction::Version),
        )
        .subcommand(
            Command::new("init")
                .about("初始化环境 | Initialize the environment")
                .arg(
                    Arg::new("dry_run")
                        .long("dry-run")
                        .help("仅模拟初始化，不实际创建文件 | Simulate initialization without creating files")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("login")
                .about("登录 MSL 用户中心")
                .arg(
                    Arg::new("email")
                        .long("email")
                        .short('e')
                        .value_name("email")
                        .help("MSL 用户中心账号邮箱")
                        .required(true),
                )
                .arg(
                    Arg::new("password")
                        .long("password")
                        .short('p')
                        .value_name("password")
                        .help("MSL 用户中心账号密码")
                        .required(true),
                )
                .arg(
                    Arg::new("twofa")
                        .short('t')
                        .long("two-factor")
                        .value_name("twoFactorAuthKey")
                        .help("MSL 用户中心 2FA 密钥 (可选)")
                )
                .arg(
                    Arg::new("save_login")
                        .short('s')
                        .long("save-login")
                        .help("保存用户 token")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(Command::new("new").about("新建服务器 (占位符)"))
        .subcommand(Command::new("list").about("列出服务器 (占位符)"))
        .subcommand(Command::new("frp").about("映射服务器 (占位符)"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub)) => {
            let dry_run = sub.get_flag("dry_run");
            match cmd_init(dry_run) {
                Ok(_) => {
                    println!("初始化成功");
                    write_log("init", "success").ok();
                    exit(0);
                }
                Err(e) => {
                    eprintln!("初始化错误: {}", e);
                    write_log("init", &format!("error: {}", e)).ok();
                    let code = if e.kind() == io::ErrorKind::PermissionDenied {
                        CODE_PERMISSION
                    } else {
                        CODE_IO
                    };
                    exit(code);
                }
            }
        }
        Some(("new", _)) => {
            cmd_new();
        }
        Some(("list", _)) => {
            cmd_list();
        }
        Some(("login", sub)) => {
                // 确认 EULA
                let eula = env::current_dir().unwrap()
                    .join("MSL").join("eula.txt");
                let agreed = fs::read_to_string(&eula)
                .map(|c| c.contains("eula=true"))
                .unwrap_or(false);
            if !agreed {
                println!("未同意用户协议 (EULA)，是否运行 `msl-cli init` 以完成初始化? (y/n)");
                let mut ans = String::new();
                io::stdin().read_line(&mut ans).unwrap();
                if ans.trim().eq_ignore_ascii_case("y") {
                    cmd_init(false).unwrap_or_else(|e| {
                    eprintln!("初始化失败: {}", e);
                        exit(CODE_IO);
                    });
                } else {
                    println!("请先同意用户协议后再尝试登录。");
                    exit(0);
                }
            }

            // 读参
            let email      = sub.get_one::<String>("email").unwrap();
            let password   = sub.get_one::<String>("password").unwrap();
            let twofa = sub.get_one::<String>("twofa").map(String::as_str);
            let save_login   = sub.get_flag("save_login");

            // 调用
            match login_user(email, password, twofa) {
                Ok(data) => {
                    println!("登录成功，欢迎：{}，Token：{}", data.username, data.token);

                    if save_login {
                        match Entry::new("msl-cli", email) {
                            Ok(entry) => {
                                if let Err(e) = entry.set_password(&data.token) {
                                    eprintln!("保存 Token 失败: {}", e);
                                } else {
                                    println!("Token 已安全保存到系统钥匙串。");
                                }
                            }
                            Err(e) => {
                                eprintln!("创建 Keyring 实例失败: {}", e);
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            }
        },
        _ => {
            // 无子命令时由 clap 自动处理 --version/-v/-V 和 --help/-h
        }
    }
}

fn cmd_init(dry_run: bool) -> io::Result<()> {
    let cwd = env::current_dir()?;

    // 权限检查
    let perm_test = cwd.join(".perm_test");
    match File::create(&perm_test) {
        Ok(mut f) => {
            writeln!(f, "permission check")?;
            drop(f);
            fs::remove_file(&perm_test)?;
        }
        Err(e) => {
            return Err(io::Error::new(
                e.kind(),
                format!("权限验证失败: {}", e),
            ));
        }
    }

    // 创建 MSL 根目录
    let msl_dir = cwd.join("MSL");
    if !msl_dir.exists() {
        fs::create_dir_all(&msl_dir)?;
        println!("已创建目录：MSL");
    } else {
        println!("目录已存在：MSL (跳过创建)");
    }

    // 创建 servers 和 logs 子目录
    for sub in &["servers", "logs"] {
        let subdir = msl_dir.join(sub);
        if !subdir.exists() {
            fs::create_dir_all(&subdir)?;
            println!("已创建目录：MSL/{}", sub);
        } else {
            println!("目录已存在：MSL/{} (跳过创建)", sub);
        }
    }

    // 创建 eula.txt 文件
    let eula_path = msl_dir.join("eula.txt");
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let user = whoami::username();
    let eula_content = format!(
        "# MSL-Cli EULA\n# Generated on: {}\n# User: {}\n\
         eula=false\n",
        now, user
    );

    if dry_run {
        println!("[dry-run] 会写入文件：{}\n{}", eula_path.display(), eula_content);
    } else if !eula_path.exists() {
        // 原子写入：先写临时文件再重命名
        let tmp = msl_dir.join("eula.txt.tmp");
        fs::write(&tmp, &eula_content)?;
        fs::rename(&tmp, &eula_path)?;
        println!("已生成协议文件：MSL/eula.txt");
    } else {
        println!("文件已存在：MSL/eula.txt (跳过创建)");
    }

    if dry_run {
        return Ok(());
    }

    // 读取并写入 eula.txt 内容
    let content: String = fs::read_to_string(&eula_path)?;
    println!("请您仔细阅读Minecraft Server Launcher的用户协议：https://mslmc.cn/eula.html");

    // 交互写入
    print!("\n输入 y 或 yes 视为同意，输入其他内容将退出：");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim().to_lowercase();

    if choice == "y" || choice == "yes" {
        if dry_run {
            println!("[dry-run] 会将 eula=false 改为 eula=true");
        } else {
            let updated = content.replace("eula=false", "eula=true");
            // 原子写入更新
            let tmp = msl_dir.join("eula.txt.tmp");
            fs::write(&tmp, &updated)?;
            fs::rename(&tmp, &eula_path)?;
            println!("\n已同意用户协议，eula.txt 已更新为 true");
        }
    } else {
        println!("\n未同意用户协议，程序将退出");
        write_log("init", "disagree").ok();
        exit(CODE_DISAGREE);
    }

    Ok(())
}

/// 写操作日志到 MSL/logs/YYYYMMDD.log
fn write_log(action: &str, result: &str) -> io::Result<()> {
    let cwd = env::current_dir()?;
    let log_dir = cwd.join("MSL").join("logs");
    fs::create_dir_all(&log_dir)?;
    let date = Local::now().format("%Y%m%d").to_string();
    let file = log_dir.join(format!("{}.log", date));

    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file)?;
    let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
    let user = whoami::username();
    writeln!(f, "[{}] [{}] [{}] {}", ts, user, action, result)?;
    Ok(())
}

/// new 子命令占位
fn cmd_new() {
    println!("new 子命令尚未实现");
    exit(0);
}

/// list 子命令占位
fn cmd_list() {
    println!("list 子命令尚未实现");
    exit(0);
}
