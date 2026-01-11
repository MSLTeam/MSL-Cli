use ratatui::prelude::*;
use crate::ui::{AppState, components};

pub fn render(f: &mut Frame, state: &AppState) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(15),
            Constraint::Min(0),
        ])
        .split(f.size());

    components::render_sidebar(f, layout[0], state);

    let (title, content) = if let Some(data) = &state.home_data {
        (data.title.as_str(), data.content.as_str())
    } else {
        ("正在加载...", "请稍后，正在获取 MSL 公告...")
    };

    components::render_banner(f, layout[1], content);
}