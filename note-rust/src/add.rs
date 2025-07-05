use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Borders, ListItem, Paragraph, Wrap},
};

use crate::settings::{append_note_to_file, note_body_input, note_title_input, NoteFormat};

pub fn draw_add_popup_title(
    f: &mut Frame,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    add_popup_active: &mut i8,
    action: &mut bool,
    error_popup_active: &mut bool,
    error_title: &mut String,
    error_text: &mut String,
) {
    let area = note_body_input(60, f.area());
    let title_len = note.text.chars().count();
    let title_style = if title_len > 100 {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(format!("Add note title ({} / 100)", title_len))
        .title_style(title_style)
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(note.text.as_str())
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);

    match key_event.code {
        KeyCode::Enter => {
            if !note.text.trim().is_empty() && note.text.len() <= 101 {
                *add_popup_active = 2;
            } else {
                if note.text.len() > 101 {
                    *error_popup_active = true;
                    *error_title = "Error".to_string();
                    *error_text = "The title must not exceed 100 characters.".to_string();
                }
                *add_popup_active = 0;
                *action = false;
            }
        }
        KeyCode::Esc => {
            *action = false;
            *add_popup_active = 0;
            note.text.clear();
        }
        KeyCode::Backspace => {
            note.text.pop();
        }
        KeyCode::Char(c) => {
            note.text.push(c);
        }
        _ => {}
    }
}

pub fn draw_add_popup_body(
    f: &mut Frame,
    note: &mut NoteFormat,
    notes: &mut Vec<NoteFormat>,
    key_event: KeyEvent,
    items: &mut Vec<ListItem>,
    add_popup_active: &mut i8,
    action: &mut bool,
    line_cnt: &mut u32,
    error_popup_active: &mut bool,
    error_title: &mut String,
    error_text: &mut String,
) -> Result<()> {
    let body_len = note.body.chars().count();
    let body_style = if body_len > 100 {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(format!("Add note body ({} / 100)", body_len))
        .title_style(body_style)
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(note.body.as_str())
        .wrap(Wrap { trim: false })
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);
    let area = note_body_input(60, f.area());
    f.render_widget(paragraph, area);

    match key_event.code {
        KeyCode::Enter => {
            if note.body.len() <= 101 {
                *line_cnt = (notes.len() + 1) as u32;
                append_note_to_file(&note.text, &note.body)?;
                items.push(ListItem::new(format!(
                    "{}: \"{}\" - \"{}\"",
                    line_cnt, note.text, note.body
                )));
                notes.push(note.clone());
                *note = NoteFormat::default();
                *action = false;
                *add_popup_active = 0;
            } else {
                *error_popup_active = true;
                *error_title = "Error".to_string();
                *error_text = "Please enter a body with 100 characters or fewer.".to_string();
            }
        }
        KeyCode::Esc => {
            *add_popup_active = 0;
            *action = false;
            note.text.clear();
            note.body.clear();
        }
        KeyCode::Backspace => {
            note.body.pop();
        }
        KeyCode::Char(c) => {
            note.body.push(c);
        }
        _ => {}
    }
    Ok(())
}
pub fn add_command(
    //TODO: fix cmd_help
    //HACK: More fast
    f: &mut Frame,
    add_popup_active: &mut i8,
    notes: &mut Vec<NoteFormat>,
    items: &mut Vec<ListItem>,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    action: &mut bool,
    line_cnt: &mut u32,
    error_popup_active: &mut bool,
    error_title: &mut String,
    error_text: &mut String,
) -> Result<()> {
    *action = true;

    match *add_popup_active {
        1 => {
            draw_add_popup_title(
                f,
                note,
                key_event,
                add_popup_active,
                action,
                error_popup_active,
                error_title,
                error_text,
            );
        }
        2 => {
            draw_add_popup_body(
                f,
                note,
                notes,
                key_event,
                items,
                add_popup_active,
                action,
                line_cnt,
                error_popup_active,
                error_title,
                error_text,
            )?;
        }
        _ => {}
    }
    Ok(())
}
