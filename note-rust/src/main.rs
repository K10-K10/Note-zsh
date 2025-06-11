use color_eyre::eyre::Result;
use crossterm::{
    cursor::{MoveDown, MoveTo, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    style::Print,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io::stdout;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    time::Duration,
};
const WIDTH: u16 = 2;
const HEIGHT: u16 = 1;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let popup_height = r.height * percent_y / 100;
    let popup_x = r.x + (r.width - popup_width) / 2;
    let popup_y = r.y + (r.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}

fn get_user_input(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<String> {
    let mut input = String::new();

    loop {
        terminal.draw(|f| {
            let area = centered_rect(60, 20, f.size());
            let block = Block::default().title("New Note").borders(Borders::ALL);
            let paragraph = Paragraph::new(Text::from(input.as_str())).block(block);

            f.render_widget(paragraph, area);
        })?;

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind != KeyEventKind::Press {
                continue;
            }
            match code {
                KeyCode::Enter => return Ok(input),
                KeyCode::Esc => return Ok(String::new()),
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                _ => {}
            }
        }
    }
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

fn draw_add_popup(
    f: &mut Frame,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let note = get_user_input(terminal)?;

    if !note.trim().is_empty() {
        append_note_to_file("note.txt", &note)?;
        let notes = load_notes("note.txt")?; //TODO: Chose pop or All load again
    }

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut notes = load_notes("note.txt")?;
    let mut items: Vec<ListItem> = notes.iter().map(|n| ListItem::new(n.as_str())).collect();
    let add_popup_active = false as bool;
    let mut add_popup_active = false;
    let mut popup_input = String::new();
    loop {
        terminal.draw(|f| {
            draw_main_ui(f, &items);
            if add_popup_active {
                let area = centered_rect(60, 20, f.size());
                let block = Block::default().title("New Note").borders(Borders::ALL);
                let paragraph = Paragraph::new(popup_input.as_str()).block(block);
                f.render_widget(paragraph, area);
            }
        })?;

        if add_popup_active {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        if !popup_input.trim().is_empty() {
                            append_note_to_file("note.txt", &popup_input)?;
                            notes = load_notes("note.txt")?;
                            items = notes.iter().map(|n| ListItem::new(n.as_str())).collect();
                        }
                        popup_input.clear();
                        add_popup_active = false;
                    }
                    KeyCode::Esc => {
                        popup_input.clear();
                        add_popup_active = false;
                    }
                    KeyCode::Backspace => {
                        popup_input.pop();
                    }
                    KeyCode::Char(c) => {
                        popup_input.push(c);
                    }
                    _ => {}
                }
            }
        } else {
            if event::poll(Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => {
                            add_popup_active = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
