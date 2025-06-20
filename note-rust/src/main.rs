use color_eyre::eyre::Result;
use crossterm::{
    // cursor::{MoveDown, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    //style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    prelude::*,
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};

use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    time::Duration,
};

static file_path: &str = "../note.txt";

fn note_title_input(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 150;
    let popup_height = 3;
    let popup_x = r.x + (r.width - popup_width) / 2;
    let popup_y = r.y + (r.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}

fn note_body_input(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 150;
    let popup_height = 9;
    let popup_x = r.x + (r.width - popup_width) / 2;
    let popup_y = r.y + (r.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}

fn load_notes() -> Result<Vec<String>> {
    let file = File::open(file_path).unwrap_or_else(|_| File::create(file_path).unwrap());
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}
fn append_note_to_file(note: &str, body: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    writeln!(file, "{:<100}", note)?;
    writeln!(file, "{:<100}", body)?;
    Ok(())
}

fn draw_main_ui(f: &mut Frame, items: &Vec<ListItem>, list_state: &mut ListState) {
    let size = f.area();

    let list_block_area = Rect::new(0, 0, size.width, size.height - 3);
    let cmd_block_area = Rect::new(0, size.height - 3, size.width, 3);

    let list = List::new(items.clone()).block(
        Block::default()
            .title("[1]: Notes")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL),
    );
    let list = List::new(items.clone())
        .highlight_symbol(">> ")
        .highlight_style(Style::default().bg(Color::Blue));

    f.render_stateful_widget(list, list_block_area, list_state);

    let cmd_block = Block::default()
        .title("")
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let cmd_paragraph = Paragraph::new(Text::from(
        "j : page down | k : page up | q : quit | a : add note | e : edit command | Enter : edit selected note",
    ))
    .block(cmd_block.clone());
    f.render_widget(cmd_block, cmd_block_area);
    f.render_widget(cmd_paragraph, cmd_block_area);
}

fn draw_add_popup_title(
    f: &mut Frame,
    area: Rect,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    add_popup_active: &mut i8,
    action: &mut bool,
) {
    let area = note_title_input(60, 20, f.area());
    let block = Block::default()
        .title("New Note Title")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(note.text.as_str()).block(block);
    f.render_widget(paragraph, area);

    match key_event.code {
        KeyCode::Enter => {
            if !note.text.trim().is_empty() && note.text.len() <= 101 {
                //TODO: add error message
                *add_popup_active = 2;
            } else {
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

fn draw_add_popup_body(
    f: &mut Frame,
    area: Rect,
    note: &mut NoteFormat,
    notes: &mut Vec<NoteFormat>,
    key_event: KeyEvent,
    items: &mut Vec<ListItem>,
    add_popup_active: &mut i8,
    action: &mut bool,
    line_cnt: &mut u32,
) -> Result<()> {
    let block = Block::default()
        .title("New Note Body")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(note.body.as_str())
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);
    let area = note_body_input(60, 20, f.area());
    f.render_widget(paragraph, area);

    match key_event.code {
        KeyCode::Enter => {
            if note.body.len() <= 101 {
                //TODO: add error message
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
fn add_command(
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
) -> Result<()> {
    *action = true;
    let area = note_body_input(60, 20, f.area());

    match *add_popup_active {
        1 => {
            draw_add_popup_title(f, area, note, key_event, add_popup_active, action);
        }
        2 => {
            draw_add_popup_body(
                f,
                area,
                note,
                notes,
                key_event,
                items,
                add_popup_active,
                action,
                line_cnt,
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn edit_line_input(
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
) {
    //TODO: when it is started, edit_line_num is set 0
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
            edit_line_num.clear(); //TODO: The variable EDIT_LINE_NUM is not clean.
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
fn edit_text_input(
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
        // eprintln!(
        //     "line_num {} is out of range for notes length {}",
        //     line_num,
        //     notes.len()
        // );

        *edit_line_num = String::new();
        *edit_popup_active = 1;
        *action = false;
        None
    };

    let block = Block::default()
        .title("Edit note title")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(note.text.as_str())
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);
    f.render_widget(paragraph, area);

    match key_event.code {
        KeyCode::Enter => {
            if !note.text.trim().is_empty() && note.text.len() <= 101 {
                //TODO: add error message
                *edit_popup_active = 3;
            } else {
                *edit_popup_active = 0;
                *action = false;
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

fn edit_body_input(
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
) -> std::io::Result<()> {
    let mut line_num = match edit_line_num.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= line_cnt as usize => n - 1,
        _ => {
            *edit_popup_active = 0;
            *action = false;
            return Ok(());
        }
    };
    let selected_note = &notes[line_num];
    let block = Block::default()
        .title("Edit note body")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(note.body.as_str()).block(block);
    f.render_widget(paragraph, area);
    match key_event.code {
        KeyCode::Enter => {
            //TODO: add error message
            if note.body.len() <= 101 {
                notes[line_num].text = note.text.clone();
                notes[line_num].body = note.body.clone();

                let mut file = std::fs::OpenOptions::new().write(true).open(file_path)?;
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

fn edit_command(
    f: &mut Frame,
    edit_popup_active: &mut i8,
    notes: &mut Vec<NoteFormat>,
    items: &mut Vec<ListItem>,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    action: &mut bool,
    line_cnt: u32,
    edit_line_num: &mut String,
) -> Result<()> {
    *action = true;
    let area = note_title_input(60, 20, f.area());
    let text_area = note_body_input(60, 20, f.area());
    match *edit_popup_active {
        1 => {
            edit_line_input(
                f,
                edit_popup_active,
                notes,
                items,
                note,
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
                items,
                note,
                key_event,
                action,
                line_cnt,
                area,
                edit_line_num,
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
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn edit_from_list(
    f: &mut Frame,
    edit_from_list_active: &mut i8,
    notes: &mut Vec<NoteFormat>,
    items: &mut Vec<ListItem>,
    note: &mut NoteFormat,
    key_event: KeyEvent,
    action: &mut bool,
    line_cnt: u32,
    edit_line_num: &mut String,
) -> Result<()> {
    *action = true;
    let area = note_title_input(60, 20, f.area());
    let text_area = note_body_input(60, 20, f.area());
    match *edit_from_list_active {
        2 => {
            edit_text_input(
                f,
                edit_from_list_active,
                notes,
                items,
                note,
                key_event,
                action,
                line_cnt,
                area,
                edit_line_num,
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
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn find_command() {}

fn filter_command() {}

#[derive(Clone, Default)]
struct NoteFormat {
    text: String,
    body: String,
}

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
    // let mut items: Vec<ListItem> = notes_raw
    // .iter()
    // .map(|n| ListItem::new(n.as_str()))
    // .collect();

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

    let mut action = false;
    let mut add_popup_active = 0;
    let mut edit_popup_active: i8 = 0;
    let mut edit_from_list_active: i8 = 0;
    let mut edit_line_num: String = "".to_string();

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
                    KeyCode::Char('F') => {
                        if !action {
                            filter_command();
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('f') => {
                        if !action {
                            find_command();
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Char('e') => {
                        if !action {
                            edit_popup_active = 1;
                            //edit_command();
                        } else {
                            key_event = Some(key);
                        }
                    }
                    KeyCode::Esc => {
                        if !action {
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
            draw_main_ui(f, &items, &mut list_state);

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
                );
            }
        })?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
