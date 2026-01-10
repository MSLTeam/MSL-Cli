mod init;
mod config;

use console::style;

#[tokio::main]
async fn main() {
    // 预启动检查
    if let Err(e) = init::run_preflight_checks() {
        eprintln!("\n{} {}", style("致命错误").red().bold(), e);
        std::process::exit(1);
    }
    
    // 检查通过后的逻辑
    println!("\n{}", style("检查通过，准备启动控制台...").green().bold());
    
    // TODO: 初始化TUI和日志收集
    start_tui().await;
}

async fn start_tui() {
    // 只是个占位
    println!("{}", style("TUI 模块建设中").magenta());
}