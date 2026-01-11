use ratatui::{prelude::*, widgets::*};

pub fn render_banner(f: &mut Frame, area: Rect, content: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let announcement = Paragraph::new(Line::from(vec![
        Span::styled("公告：", Style::default().bold().fg(Color::Yellow)),
        Span::raw(content),
    ]));
    f.render_widget(announcement, chunks[0]);

    let image_placeholder = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .title(" Image Area ");
    f.render_widget(image_placeholder, chunks[1])
}