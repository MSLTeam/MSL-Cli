mod init;
mod config;
mod ui;
mod tui;
mod core;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;
use tokio::sync::mpsc;
use crate::core::api::{ApiClient, AnnouncementData};
use console::style;

pub enum IoEvent {
    HomeDataLoaded(AnnouncementData),
    LoadError(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    // 预启动检查
    if let Err(e) = init::run_preflight_checks() {
        eprintln!("\n{} {}", style("致命错误").red().bold(), e);
        std::process::exit(1);
    }
    
    // 检查通过后的逻辑
    println!("\n{}", style("检查通过，准备启动控制台...").green().bold());

    // 异步获取数据
    let (tx, mut rx) = mpsc::channel(10);

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let api = ApiClient::new();
        match api.fetch_home_announcement().await {
            Ok(data) => {
                let _ = tx_clone.send(IoEvent::HomeDataLoaded(data)).await;
            }
        }
    });

    // 初始化 TUI
    let mut terminal = tui::init()?;
    let mut app_state = ui::AppState::new();

    // 主事件循环
    loop {
        while let Ok(io_event) = rx.try_recv() {
            match io_event {
                IoEvent::HomeDataLoaded(data) => {
                    app_state.home_data = Some(crate::core::api::Announcement {
                       data: "".to_string(),
                    });
                }
                IoEvent::LoadError(err) => {
                    // 处理逻辑错误，可以在 UI 状态记录错误信息
                }
            }
        }

        terminal.draw(|f| ui::render(f, &app_state))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app_state.should_quit = true,
                        KeyCode::Down => app_state.selected_tab = (app_state.selected_tab + 1 ) % 6,
                        KeyCode::Up => app_state.selected_tab = if app_state.selected_tab == 0 { 5 } else { app_state.selected_tab - 1 },
                        _ => {}
                    }
                }
            }
        }

        if app_state.should_quit {
            break;
        }
    }

    tui::restore()?;
    Ok(())
}