use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    style::{Color, Style},
    text::{Text, ToText},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::{
    io::{self},
    time::Duration,
};

use crate::settings::note_title_input;

pub fn move_command(
    f: &mut Frame,
    key_event: KeyEvent,
    action: &mut bool,
    move_popup_active: &mut bool,
    line_cnt: u32,
    list_state: &mut ListState,
    move_line: &mut String,
) -> std::io::Result<()> {
    *action = true;
    let area = note_title_input(60, f.area());
    let move_block = Block::default()
        .title("Move to line")
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);
    let cmd_paragraph = Paragraph::new(Text::from(move_line.as_str())).block(move_block);
    f.render_widget(cmd_paragraph, area);
    match key_event.code {
        KeyCode::Enter => {
            if let Ok(index) = move_line.trim().parse::<usize>() {
                if index >= 1 && index <= line_cnt as usize {
                    list_state.select(Some(index - 1));
                    *move_popup_active = false;
                    *action = false;
                    move_line.clear();
                } else {
                    //TODO: popup error
                    *move_popup_active = false;
                    *action = false;
                    move_line.clear();
                }
            }
        }
        KeyCode::Esc => {
            *move_popup_active = false;
            move_line.clear();
            *action = false;
        }
        KeyCode::Backspace => {
            move_line.pop();
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            move_line.push(c);
        }
        KeyCode::Right => {
            *move_line = (line_cnt / 2).to_string();
        }
        _ => {}
    }

    Ok(())
}
