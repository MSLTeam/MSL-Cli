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

    if let Some(data) = &state.home_data {
        let tips_text = if !data.tips.is_empty() {
            format!(" Tips: {}", data.tips.join(" | "))
        } else {
            String::new()
        };
        
        components::render_banner(f, layout[1], "公告", &tips_text, &data.notice_html);
    }
}