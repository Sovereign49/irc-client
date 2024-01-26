use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let messages = app.messages.join("\n");
    let width = frame.size().width;
    let height = frame.size().height;
    frame.render_widget(
        Paragraph::new(format!("{}", messages))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::LightBlue))
            .alignment(Alignment::Left),
        Rect::new(0, 0, width, height - 4),
    );
    let user_msg = app.user_msg.clone();
    frame.render_widget(
        Paragraph::new(user_msg)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::LightBlue))
            .alignment(Alignment::Left),
        Rect::new(0, height - 4, width, 4),
    );
}
