use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::Rect;
use ratatui::{
    backend::CrosstermBackend,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let _size = terminal.size()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let List_block_mix = 0 as u16;
            let List_block_miy = 0 as u16;
            let List_block_x = size.width / 5 * 3 as u16;
            let List_block_y = size.height - 4 as u16;
            let cmd_block_mix = 0 as u16;
            let cmd_block_miy = size.height - 3 as u16;
            let cmd_block_x = size.width as u16;
            let cmd_block_y = size.height as u16;

            let List_block = Block::default().title("[1]: Notes").borders(Borders::ALL); // TODOL Change for list
            let List_block_area =
                Rect::new(List_block_mix, List_block_miy, List_block_x, List_block_y);
            f.render_widget(&List_block, List_block_area);
            let List_block_paragraph =
                Paragraph::new(Text::from("Hello TUI!")).block(List_block.clone());
            f.render_widget(List_block_paragraph, List_block_area);

            let cmd_block = Block::default().title("cmd").borders(Borders::ALL);
            let cmd_block_area = Rect::new(cmd_block_mix, cmd_block_miy, cmd_block_x, cmd_block_y);
            f.render_widget(&cmd_block, cmd_block_area);
            let cmd_block_paragraph =
                Paragraph::new(Text::from("q: quit")).block(cmd_block.clone());
            f.render_widget(cmd_block_paragraph, cmd_block_area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            } else if key.code == KeyCode::Char('a') {
                //TODO: add
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
