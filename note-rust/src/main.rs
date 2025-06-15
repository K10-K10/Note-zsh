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
    prelude::*,
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    time::Duration,
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let popup_height = r.height * percent_y / 100;
    let popup_x = r.x + (r.width - popup_width) / 2;
    let popup_y = r.y + (r.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}

fn load_notes(path: &str) -> Result<Vec<String>> {
    let file = File::open(path).unwrap_or_else(|_| File::create(path).unwrap());
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}
fn append_note_to_file(path: &str, note: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{}", note)?;
    Ok(())
}

fn draw_main_ui(f: &mut Frame, items: &Vec<ListItem>) {
    let size = f.area();

    let list_block_area = Rect::new(0, 0, size.width * 3 / 5, size.height - 3);
    let cmd_block_area = Rect::new(0, size.height - 3, size.width, 3);

    let list = List::new(items.clone()).block(
        Block::default()
            .title("[1]: Notes")
            .border_type(ratatui::widgets::BorderType::Rounded)
            .borders(Borders::ALL),
    );
    f.render_widget(list, list_block_area);

    let cmd_block = Block::default() //TODO: cmd line show when push 'h' key
        .title("")
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL);
    let cmd_paragraph =
        Paragraph::new(Text::from("q: quit / a: add note")).block(cmd_block.clone()); // HACK: make variable that cmd-text
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
    let block = Block::default()
        .title("New Note Title")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(note.text.as_str()).block(block);
    f.render_widget(paragraph, area);

    match key_event.code {
        KeyCode::Enter => {
            if !note.text.trim().is_empty() {
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
    let paragraph = Paragraph::new(note.body.as_str()).block(block);
    f.render_widget(paragraph, area);

    match key_event.code {
        KeyCode::Enter => {
            *line_cnt = (*line_cnt + 2) / 2;
            append_note_to_file("../note.txt", &note.text)?;
            append_note_to_file("../note.txt", &note.body)?;
            items.push(ListItem::new(format!(
                "{}: \"{}\" - \"{}\"",
                line_cnt, note.text, note.body
            )));
            notes.push(note.clone());
            *note = NoteFormat::default();
            *action = false;
            *add_popup_active = 0;
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
    let area = centered_rect(60, 20, f.area());

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

fn edit_command() {
    //TODO : Make edit command
}

fn find_command() {}

fn filter_command() {}

#[derive(Clone, Default)]
struct NoteFormat {
    text: String,
    body: String,
}

enum KeyCommand {
    Q,
    A,
    L,
    F,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let notes_raw: Vec<String> = load_notes("../note.txt")?;
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

    let mut note = NoteFormat::default();
    let mut keycnt = false as bool;

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
                            edit_command();
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
            draw_main_ui(f, &items);

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
        })?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
