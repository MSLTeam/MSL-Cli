use std::fs;
use std::path::Path;
use anyhow::{Result, bail};
use console::style;
use serde::{Serialize, Deserialize};
use crate::init::paths::*;
use crate::config::MslConfig;

#[derive(Serialize, Deserialize)]
struct Eula {
    eula: bool,
}

pub fn run_preflight_checks() -> Result<()> {
    println!("{}", style(">> [1/4] 正在检查文件系统权限...").cyan());
    check_write_permission(".")?;
    
    println!("{}", style(">> [2/4] 正在初始化工作目录...").cyan());
    ensure_dirs(&[DIR_CONFIGS,DIR_MSL_CONF,DIR_LOGS,DIR_SERVERS])?;
    
    println!("{}", style(">> [3/4] 正在验证 EULA 协议状态...").cyan());
    check_eula()?;
    
    println!("{}", style(">> [4/4] 正在校验配置文件...").cyan());
    validate_configs()?;
    
    Ok(())
}

fn check_write_permission(path: &str) -> Result<()> {
    let test_path = Path::new(path).join(".msl_perm_test");
    if let Err(_) = fs::write(&test_path, "") {
        bail!(style("权限错误：无法在当前目录写入，请确保有足够的读写权限。").red().bold());
    }
    let _ = fs::remove_file(test_path);
    Ok(())
}

fn ensure_dirs(dirs: &[&str]) -> Result<()> {
    for dir in dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}
fn check_eula() -> Result<()> {
    let path = Path::new(FILE_EULA);
    if !path.exists() {
        let default_eula = Eula { eula: false };
        fs::write(path, serde_json::to_string_pretty(&default_eula)?)?;
        prompt_eula_and_exit();
    }
    
    let content = fs::read_to_string(path)?;
    let eula: Eula = serde_json::from_str(&content)?;
    
    if !eula.eula {
        prompt_eula_and_exit();
    }
    Ok(())
}

fn prompt_eula_and_exit() -> ! {
    println!("\n{}", style("--------------------------------------------------").yellow());
    println!("{}", style("您必须阅读并同意 Minecraft Server Launcher Eula 协议才能继续。").bold());
    println!("协议地址：{}", style("https://mslmc.net/eula").blue().underlined());
    println!("\n若您同意，请将以下文件中的 eula 设置为 true: ");
    println!("{}", style(FILE_EULA).green());
    println!("{}", style("--------------------------------------------------").yellow());
    std::process::exit(0);
}

fn validate_configs() -> Result<()> {
    let path = Path::new(FILE_CONFIG);
    if !path.exists() {
        let default_conf = MslConfig::default();
        fs::write(path, serde_json::to_string_pretty(&default_conf)?)?;
        println!("{}", style("已生成默认配置文件，请根据需求修改后再启动。").yellow());
        std::process::exit(0);
    }
    
    let content = fs::read_to_string(path)?;
    if let Err(e) = serde_json::from_str::<MslConfig>(&content) {
        bail!(style(format!("配置错误：{} 格式校验失败!\n原因：{}", FILE_CONFIG, e)).red());
    }
    Ok(())
}
