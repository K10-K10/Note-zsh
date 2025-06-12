use color_eyre::eyre::Result;
use crossterm::{
    cursor::{MoveDown, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    //style::Print,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
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

    let cmd_block = Block::default()
        .title("")
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL);
    let cmd_paragraph =
        Paragraph::new(Text::from("q: quit / a: add note")).block(cmd_block.clone());
    f.render_widget(cmd_block, cmd_block_area);
    f.render_widget(cmd_paragraph, cmd_block_area);
}

fn add_command(
    f: &mut Frame,
    add_popup_active: &mut i8,
    notes: &mut Vec<String>,
    items: &mut Vec<ListItem>,
    note: &mut NoteFormat,
    key_event: KeyEvent,
) -> Result<()> {
    let area = centered_rect(60, 20, f.area());

    if *add_popup_active == 1 {
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
                    *add_popup_active = 0; // 空ならキャンセル
                }
            }
            KeyCode::Esc => {
                note.text.clear();
                *add_popup_active = 0;
            }
            KeyCode::Backspace => {
                note.text.pop();
            }
            KeyCode::Char(c) => {
                note.text.push(c);
            }
            _ => {}
        }
    } else if *add_popup_active == 2 {
        let block = Block::default()
            .title("New Note Body")
            .borders(Borders::ALL);
        let paragraph = Paragraph::new(note.body.as_str()).block(block);
        f.render_widget(paragraph, area);

        match key_event.code {
            KeyCode::Enter => {
                append_note_to_file("note.txt", &note.text)?;
                append_note_to_file("note.txt", &note.body)?;
                *notes = load_notes("note.txt")?;
                *items = notes.iter().map(|n| ListItem::new(n.clone())).collect();
                note.text.clear();
                note.body.clear();
                *add_popup_active = 0;
            }
            KeyCode::Esc => {
                note.text.clear();
                note.body.clear();
                *add_popup_active = 0;
            }
            KeyCode::Backspace => {
                note.body.pop();
            }
            KeyCode::Char(c) => {
                note.body.push(c);
            }
            _ => {}
        }
    }

    Ok(())
}

struct NoteFormat {
    text: String,
    body: String,
}

union key_command {
    q: bool,
    a: bool,
    l: bool,
    f: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut notes: Vec<String> = load_notes("note.txt")?;
    let mut notes_2: Vec<String> = load_notes("note.txt")?;
    let mut add_popup_active = 0i8;
    let mut note = NoteFormat {
        text: String::new(),
        body: String::new(),
    };
    let mut items: Vec<ListItem> = notes_2.iter().map(|n| ListItem::new(n.as_str())).collect();

    loop {
        let mut key_event = None;
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => {
                        if (add_popup_active == 0) {
                            add_popup_active = 1;
                        }
                    }
                    _ => {}
                }
                key_event = Some(key);
            }
        }

        terminal.draw(|f| {
            draw_main_ui(f, &items);

            // key_event があるときだけ popup を処理して描画
            if let Some(k) = key_event {
                let _ = add_command(
                    f,
                    &mut add_popup_active,
                    &mut notes,
                    &mut items,
                    &mut note,
                    k,
                );
            }

            // key_event が None のときも popup を表示し続けるために呼ぶ！
            if key_event.is_none() && add_popup_active != 0 {
                let dummy_key = KeyEvent::new(KeyCode::Null, event::KeyModifiers::NONE);
                let _ = add_command(
                    f,
                    &mut add_popup_active,
                    &mut notes,
                    &mut items,
                    &mut note,
                    dummy_key,
                );
            }
        })?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
