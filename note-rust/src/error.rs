use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::settings::note_title_input;

pub fn error_command(
    f: &mut Frame,
    error_popup_active: &mut bool,
    error_title: String,
    error_text: String,
    key_event: KeyEvent,
) -> Result<()> {
    *error_popup_active = true;
    let area = note_title_input(60, f.area());
    let error_block = Block::default()
        .title(error_title)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let error_paragraph = Paragraph::new(Text::from(error_text))
        .block(error_block.clone())
        .style(Style::default().fg(Color::Red));
    f.render_widget(error_block, area);
    f.render_widget(error_paragraph, area); //PIN: error popup

    match key_event.code {
        KeyCode::Char('q') => {
            *error_popup_active = false;
        }
        KeyCode::Esc => {
            *error_popup_active = false;
        }
        _ => (),
    }
    Ok(())
}
