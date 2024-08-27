use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;
use rust_i18n::t;

/// Render a blue screen of death.
pub fn render(f: &mut Frame<'_>, rect: Rect, error: &str) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(rect);
    let block = Block::default()
        .title(t!("title_hops").to_string())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Blue));
    let line = vec![
        Line::from(Span::styled(
            t!("bsod_failed"),
            Style::default().add_modifier(Modifier::REVERSED),
        )),
        Line::from(""),
        Line::from(error),
        Line::from(""),
        Line::from(t!("bsod_quit").to_string()),
    ];
    let paragraph = Paragraph::new(line).alignment(Alignment::Center);
    f.render_widget(block, rect);
    f.render_widget(paragraph, chunks[1]);
}
