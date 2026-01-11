use ratatui::{prelude::*, widgets::*};

pub fn render_banner(f: &mut Frame, area: Rect, title: &str, tips: &str, content: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(area);

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(chunks[0]);

    let content_widget = Paragraph::new(content)
        .wrap(Wrap { trim: true })
        .block(Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Rgb(50, 50, 50)))
            .padding(Padding::new(0, 0, 1, 0))
        );

    f.render_widget(content_widget, chunks[1]);

    let title_widget = Paragraph::new(Span::styled(
        title,
        Style::default().bold().fg(Color::Cyan)
    ));

    let tips_widget = Paragraph::new(Span::styled(
        tips,
        Style::default().fg(Color::DarkGray).italic()
    )).alignment(Alignment::Right);

    f.render_widget(title_widget, header_chunks[0]);
    f.render_widget(tips_widget, header_chunks[2]);
}