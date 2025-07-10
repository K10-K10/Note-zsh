use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{ListItem, ListState},
    Terminal,
};
use std::{
    io::{self},
    time::Duration,
};

mod add;
mod delete;
mod draw_ui;
mod edit;
mod error;
mod filter;
mod movement;
mod settings;

use crate::add::add_command;
use crate::delete::{delete_command, delete_command_check};
use crate::draw_ui::draw_main_ui;
use crate::edit::{edit_command, edit_from_list};
use crate::error::error_command;
use crate::movement::move_command;
use crate::settings::{info_command, load_notes, NoteFormat};

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let notes_raw: Vec<String> = load_notes()?
        .into_iter()
        .map(|line| line.trim_end().to_string())
        .collect();
    let mut line_cnt = notes_raw.len() as u32;
    let mut notes: Vec<NoteFormat> = vec![];
    let mut items: Vec<ListItem<'_>> = vec![];
    let mut i = 0;
    while i + 1 < notes_raw.len() {
        let note = NoteFormat {
            text: notes_raw[i].clone(),
            body: notes_raw[i + 1].clone(),
        };
        items.push(ListItem::new(format!(
            "{}: \"{}\" - \"{}\"",
            ((i + 2) / 2),
            note.text,
            note.body
        )));
        notes.push(note);
        i += 2;
    }
    let version = info_command().unwrap_or("unknown".to_string());
    let cmd_text = format!(
    " {} | d : delete line | a : add note | e : edit | Enter : edit selected note | (q esc) : quit",
    version
);

    let mut action: bool = false; //TODO: Use union
    let mut add_popup_active: i8 = 0;
    let mut edit_popup_active: i8 = 0;
    let mut edit_from_list_active: i8 = 0;
    let mut edit_line_num: String = "".to_string();
    let mut error_popup_active: bool = false;
    let mut move_popup_active: bool = false;
    let mut delete_popup_active: i8 = 0;
    let mut delete_line: String = "".to_string();
    let mut filter_popup_active: bool = false;
    let mut filter_query: String = "".to_string();
    let mut filter_result: Vec<ListItem<'_>> = vec![];
    let mut move_line: String = "".to_string();
    let mut error_title: String = "".to_string();
    let mut error_text: String = "".to_string();

    let mut note = NoteFormat::default();

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        let mut key_event = None;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('a') => {
                        if !action {
                            if add_popup_active == 0 {
                                add_popup_active = 1;
                            }
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('q') => {
                        if !action {
                            break;
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('d') => {
                        if !action {
                            delete_popup_active = 1;
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('e') => {
                        if !action {
                            edit_popup_active = 1;
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('m') => {
                        if !action {
                            move_popup_active = true;
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('f') => {
                        if !action {
                            filter_query = String::new();
                            filter_popup_active = true;
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Esc => {
                        if !action && !filter_popup_active {
                            break;
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Down => {
                        if !action {
                            let i = list_state.selected().unwrap_or(0);
                            let new_i = if i + 1 >= items.len() { 0 } else { i + 1 };
                            list_state.select(Some(new_i));
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Up => {
                        if !action {
                            let i = list_state.selected().unwrap_or(0);
                            let new_i = if i == 0 { items.len() - 1 } else { i - 1 };
                            list_state.select(Some(new_i));
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('j') => {
                        if !action {
                            let i = list_state.selected().unwrap_or(0);
                            let new_i = if i + 1 >= items.len() { 0 } else { i + 1 };
                            list_state.select(Some(new_i));
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('k') => {
                        if !action {
                            let i = list_state.selected().unwrap_or(0);
                            let new_i = if i == 0 { items.len() - 1 } else { i - 1 };
                            list_state.select(Some(new_i));
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Enter => {
                        if !action {
                            if let Some(index) = list_state.selected() {
                                edit_line_num = (index + 1).to_string();
                            }
                            edit_from_list_active = 2; // NOTE: fn edit_text is used , so active set 2
                        } else {
                            key_event = Some(key);
                        }
                    }

                    _ => {
                        key_event = Some(key);
                    }
                }
            }
        }

        terminal.draw(|f| {
            if filter_popup_active {
                draw_main_ui(f, &filter_result, &mut list_state, &cmd_text, &filter_query);
            } else {
                draw_main_ui(f, &items, &mut list_state, &cmd_text, &filter_query);
            }
            let current_key = key_event.unwrap_or_else(|| {
                if add_popup_active != 0 {
                    KeyEvent::new(KeyCode::Null, event::KeyModifiers::NONE)
                } else {
                    KeyEvent::new(KeyCode::Null, event::KeyModifiers::CONTROL)
                }
            });

            if add_popup_active != 0 {
                let _ = add_command(
                    f,
                    &mut add_popup_active,
                    &mut notes,
                    &mut items,
                    &mut note,
                    current_key,
                    &mut action,
                    &mut line_cnt,
                    &mut error_popup_active,
                    &mut error_title,
                    &mut error_text,
                );
            }
            if edit_popup_active != 0 {
                let _ = edit_command(
                    f,
                    &mut edit_popup_active,
                    &mut notes,
                    &mut items,
                    &mut note,
                    current_key,
                    &mut action,
                    line_cnt,
                    &mut edit_line_num,
                    &mut error_popup_active,
                    &mut error_title,
                    &mut error_text,
                );
            }
            if edit_from_list_active != 0 {
                let _ = edit_from_list(
                    f,
                    &mut edit_from_list_active,
                    &mut notes,
                    &mut items,
                    &mut note,
                    current_key,
                    &mut action,
                    line_cnt,
                    &mut edit_line_num,
                    &mut error_popup_active,
                    &mut error_title,
                    &mut error_text,
                );
            }
            if error_popup_active {
                let _ = error_command(
                    f,
                    &mut error_popup_active,
                    error_title.clone(),
                    error_text.clone(),
                    current_key,
                );
            }
            if move_popup_active {
                let _ = move_command(
                    f,
                    current_key,
                    &mut action,
                    &mut move_popup_active,
                    line_cnt,
                    &mut list_state,
                    &mut move_line,
                );
            }
            if delete_popup_active == 1 {
                let _ = delete_command(
                    f,
                    &mut list_state,
                    current_key,
                    &mut action,
                    &mut delete_popup_active,
                    &mut delete_line,
                    line_cnt,
                );
            } else if delete_popup_active == 2 {
                let _ = delete_command_check(
                    f,
                    &mut items,
                    &mut notes,
                    &mut list_state,
                    current_key,
                    &mut action,
                    &mut delete_popup_active,
                );
            }
            if filter_popup_active {
                filter::filter_command(
                    f,
                    current_key,
                    &notes,
                    &mut filter_result,
                    &mut filter_query,
                    &mut filter_popup_active,
                    &mut action,
                );
            }
        })?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
