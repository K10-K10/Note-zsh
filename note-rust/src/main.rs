use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    text::Text,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Terminal,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    time::Duration,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let notes = load_notes("note.txt")?;
    let items: Vec<ListItem> = notes.iter().map(|n| ListItem::new(n.as_str())).collect();

    loop {
        terminal.draw(|f| {
            let size = f.size();

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
                Paragraph::new(Text::from("q: quit / a: popup")).block(cmd_block.clone());
            f.render_widget(cmd_block, cmd_block_area);
            f.render_widget(cmd_paragraph, cmd_block_area);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => {
                        while true {
                            terminal.draw(|f| {
                                let popup_area = centered_rect(60, 20, f.size());
                                f.render_widget(Clear, popup_area);
                                let popup = Paragraph::new("This is a popup!")
                                    .block(Block::default().title("Popup").borders(Borders::ALL));
                                f.render_widget(popup, popup_area);
                            })?;
                            if let Event::Key(key) = event::read()? {
                                if key.code == KeyCode::Char('q') {
                                    break;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn load_notes(path: &str) -> Result<Vec<String>> {
    let file = File::open(path).unwrap_or_else(|_| File::create(path).unwrap());
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let popup_height = r.height * percent_y / 100;
    let popup_x = r.x + (r.width - popup_width) / 2;
    let popup_y = r.y + (r.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}
