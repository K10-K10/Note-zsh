use crate::settings::{note_title_input, save_notes, NoteFormat};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    text::Text,
    widgets::{Block, BorderType, Borders, ListItem, ListState, Paragraph},
    Frame,
};
use std::io;

pub fn delete_command_check<'a>(
    f: &mut Frame,
    items: &mut Vec<ListItem<'a>>,
    notes: &mut Vec<NoteFormat>,
    list_state: &mut ListState,
    key: KeyEvent,
    action: &mut bool,
    delete_popup_active: &mut i8,
) -> io::Result<()> {
    let area = note_title_input(60, f.area());
    let delete_block = Block::default()
        .title("Delete? (y/n)")
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    f.render_widget(
        Paragraph::new(Text::from("Delete selected note? y/N")).block(delete_block),
        area,
    );

    match key.code {
        KeyCode::Char('y') => {
            if let Some(i) = list_state.selected() {
                items.remove(i);
                notes.remove(i);
                list_state.select(Some(i.saturating_sub(1)));
                save_notes(notes)?;
            }
            *delete_popup_active = 0;
            *action = false;
        }
        KeyCode::Char('n') | KeyCode::Esc | KeyCode::Enter => {
            *delete_popup_active = 0;
            *action = false;
        }
        _ => {}
    }

    Ok(())
}

pub fn delete_command<'a>(
    f: &mut Frame,
    list_state: &mut ListState,
    key: KeyEvent,
    action: &mut bool,
    delete_popup_active: &mut i8,
    delete_line: &mut String,
    line_cnt: u32,
) -> io::Result<()> {
    *action = true;
    let area = note_title_input(60, f.area());
    let delete_block = Block::default()
        .title("Delete line")
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    f.render_widget(
        Paragraph::new(Text::from(delete_line.as_str())).block(delete_block),
        area,
    );

    match key.code {
        KeyCode::Enter => {
            if let Ok(index) = delete_line.trim().parse::<usize>() {
                if index >= 1 && index <= line_cnt as usize {
                    list_state.select(Some(index - 1));
                    *delete_popup_active = 2;
                    *action = true;
                    delete_line.clear();
                } else {
                    *delete_popup_active = 0;
                    *action = false;
                    delete_line.clear();
                }
            }
        }
        KeyCode::Esc => {
            *delete_popup_active = 0;
            delete_line.clear();
            *action = false;
        }
        KeyCode::Backspace => {
            delete_line.pop();
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            delete_line.push(c);
        }
        KeyCode::Right => {
            *delete_line = (line_cnt / 2).to_string();
        }
        _ => {}
    }

    Ok(())
}
