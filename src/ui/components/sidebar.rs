use ratatui::{prelude::*, widgets::*};
use crate::ui::AppState;

pub fn render_sidebar(f: &mut Frame, area: Rect, state: &AppState) {
    let items = vec![
        ListItem::new(" 主页 "),
        ListItem::new(" 服务器 "),
        ListItem::new(" 映射 "),
        ListItem::new(" 联机 "),
        ListItem::new(" 设置 "),
        ListItem::new(" 关于 "),
    ];

    let list = List::new(items)
        .block(Block::default().borders(Borders::RIGHT).border_style(Style::default()).fg(Color::DarkGray))
        .highlight_style(Style::default().bg(Color::Rgb(0, 120, 215)).fg(Color::White))
        .highlight_symbol(">> ");
    
    let mut list_state = ListState::default().with_selected(Some(state.selected_tab));

    f.render_stateful_widget(list, area, &mut list_state);
}