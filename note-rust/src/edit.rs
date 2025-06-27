use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Alignment,
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Borders, ListItem, Paragraph, Wrap},
};

use crate::settings::{note_body_input, note_title_input, NoteFormat, FILE_PATH};

use std::io::{Seek, Write};

fn edit_line_input(
    f: &mut Frame,
    edit_popup_active: &mut i8,
    key_event: KeyEvent,
    action: &mut bool,
    line_cnt: u32,
    area: Rect,
    edit_line_num: &mut String,
) {
    let block = Block::default()
        .title("Edit note line number")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(edit_line_num.as_str()).block(block);
    f.render_widget(paragraph, area);
    match key_event.code {
        KeyCode::Enter => {
            let edit_line_num: u32 = edit_line_num.parse().unwrap_or(0);
            if line_cnt >= edit_line_num as u32 {
                *edit_popup_active = 2;
            } else {
                *edit_popup_active = 0;
                *action = false;
            }
        }
        KeyCode::Esc => {
            *edit_popup_active = 0;
            *action = false;
            edit_line_num.clear();
        }
        KeyCode::Backspace => {
            edit_line_num.pop();
        }
        KeyCode::Char(c) => {
            edit_line_num.push(c);
        }
        _ => {}
    }
}

pub fn edit_text_input(
    f: &mut Frame,
    edit_popup_active: &mut i8,
    notes: &mut Vec<NoteFormat>,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    action: &mut bool,
    line_cnt: u32,
    area: Rect,
    edit_line_num: &mut String,
    error_popup_active: &mut bool,
    error_title: &mut String,
    error_text: &mut String,
) {
    let line_num = match edit_line_num.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= line_cnt as usize => n - 1,
        _ => {
            *edit_popup_active = 0;
            *action = false;
            return;
        }
    };
    let selected_note: Option<&NoteFormat> = if line_num < notes.len() {
        Some(&notes[line_num])
    } else {
        *edit_line_num = String::new();
        *edit_popup_active = 1;
        *action = false;
        None
    };

    let title_len = note.text.chars().count();
    let title_style = if title_len > 100 {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(format!("Edit note title ({} / 100)", title_len))
        .title_style(title_style)
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(note.text.as_str())
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);
    f.render_widget(paragraph, area);

    match key_event.code {
        KeyCode::Enter => {
            if !note.text.trim().is_empty() && note.text.len() <= 101 {
                *edit_popup_active = 3;
            } else {
                *error_popup_active = true;
                *error_title = "Error".to_string();
                *error_text = "Please enter a title with 100 characters or fewer.".to_string();
            }
        }
        KeyCode::Esc => {
            *action = false;
            *edit_popup_active = 0;
            note.text.clear();
        }
        KeyCode::Backspace => {
            note.text.pop();
        }
        KeyCode::Char(c) => {
            note.text.push(c);
        }
        KeyCode::Right => {
            if let Some(sn) = selected_note {
                note.text = sn.text.clone();
            }
        }

        _ => {}
    }
}

pub fn edit_body_input(
    f: &mut Frame,
    edit_popup_active: &mut i8,
    notes: &mut Vec<NoteFormat>,
    items: &mut Vec<ListItem>,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    action: &mut bool,
    line_cnt: u32,
    area: Rect,
    edit_line_num: &mut String,
    error_popup_active: &mut bool,
    error_title: &mut String,
    error_text: &mut String,
) -> std::io::Result<()> {
    let line_num = match edit_line_num.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= line_cnt as usize => n - 1,
        _ => {
            *edit_popup_active = 0;
            *action = false;
            return Ok(());
        }
    };
    let selected_note = &notes[line_num];

    let body_len = note.body.chars().count();
    let body_style = if body_len > 100 {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(format!("Edit note body ({} / 100)", body_len))
        .title_style(body_style)
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(note.body.as_str()).block(block);
    f.render_widget(paragraph, area);
    match key_event.code {
        KeyCode::Enter => {
            if note.body.len() <= 101 {
                notes[line_num].text = note.text.clone();
                notes[line_num].body = note.body.clone();

                let mut file = std::fs::OpenOptions::new().write(true).open(FILE_PATH)?;
                let offset = (101 * line_num) as u64;
                file.seek(std::io::SeekFrom::Start(offset))?;
                let padded = format!("{:<100}\n", note.text);
                file.write_all(padded.as_bytes())?;
                let offset = (101 * (line_num + 1)) as u64;
                file.seek(std::io::SeekFrom::Start(offset))?;
                let padded = format!("{:<100}\n", note.body);
                file.write_all(padded.as_bytes())?;

                items[line_num] = ListItem::new(format!(
                    "{}: \"{}\" - \"{}\"",
                    line_num + 1,
                    notes[line_num].text,
                    notes[line_num].body
                ));
                *edit_popup_active = 0;
                *action = false;
                note.text.clear();
                note.body.clear();
                edit_line_num.clear();
            } else {
                *error_popup_active = true;
                *error_title = "Error".to_string();
                *error_text = "Please enter a body with 100 characters or fewer.".to_string();
            }
        }
        KeyCode::Esc => {
            *edit_popup_active = 0;
            *action = false;
            note.body.clear();
        }
        KeyCode::Backspace => {
            note.body.pop();
        }
        KeyCode::Char(c) => {
            note.body.push(c);
        }
        KeyCode::Right => {
            note.body = selected_note.body.clone();
        }
        _ => {}
    }

    Ok(())
}

pub fn edit_command(
    f: &mut Frame,
    edit_popup_active: &mut i8,
    notes: &mut Vec<NoteFormat>,
    items: &mut Vec<ListItem>,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    action: &mut bool,
    line_cnt: u32,
    edit_line_num: &mut String,
    error_popup_active: &mut bool,
    error_title: &mut String,
    error_text: &mut String,
) -> Result<()> {
    *action = true;
    let area = note_title_input(60, f.area());
    let text_area = note_body_input(60, f.area());
    match *edit_popup_active {
        1 => {
            edit_line_input(
                f,
                edit_popup_active,
                key_event,
                action,
                line_cnt,
                area,
                edit_line_num,
            );
        }
        2 => {
            edit_text_input(
                f,
                edit_popup_active,
                notes,
                note,
                key_event,
                action,
                line_cnt,
                area,
                edit_line_num,
                error_popup_active,
                error_title,
                error_text,
            );
        }
        3 => {
            edit_body_input(
                f,
                edit_popup_active,
                notes,
                items,
                note,
                key_event,
                action,
                line_cnt,
                text_area,
                edit_line_num,
                error_popup_active,
                error_title,
                error_text,
            )?;
        }
        _ => {}
    }
    Ok(())
}

pub fn edit_from_list(
    f: &mut Frame,
    edit_from_list_active: &mut i8,
    notes: &mut Vec<NoteFormat>,
    items: &mut Vec<ListItem>,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    action: &mut bool,
    line_cnt: u32,
    edit_line_num: &mut String,
    error_popup_active: &mut bool,
    error_title: &mut String,
    error_text: &mut String,
) -> Result<()> {
    *action = true;
    let area = note_title_input(60, f.area());
    let text_area = note_body_input(60, f.area());
    match *edit_from_list_active {
        2 => {
            edit_text_input(
                f,
                edit_from_list_active,
                notes,
                note,
                key_event,
                action,
                line_cnt,
                area,
                edit_line_num,
                error_popup_active,
                error_title,
                error_text,
            );
        }
        3 => {
            edit_body_input(
                f,
                edit_from_list_active,
                notes,
                items,
                note,
                key_event,
                action,
                line_cnt,
                text_area,
                edit_line_num,
                error_popup_active,
                error_title,
                error_text,
            )?;
        }
        _ => {}
    }
    Ok(())
}
