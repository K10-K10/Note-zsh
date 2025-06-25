use crate::settings::{note_title_input, NoteFormat};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    text::Text,
    widgets::{Block, BorderType, Borders, ListItem, Paragraph},
    Frame,
};

pub fn filter_command<'a>(
    f: &mut Frame,
    key: KeyEvent,
    notes: &Vec<NoteFormat>,
    items: &mut Vec<ListItem<'a>>,
    filter_result: &mut Vec<ListItem<'a>>,
    query: &mut String,
    filter_popup_active: &mut bool,
) {
    let area = note_title_input(60, f.area());
    let block = Block::default()
        .title("Filter notes")
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);
    let para = Paragraph::new(Text::from(query.clone())).block(block);
    f.render_widget(para, area);

    match key.code {
        KeyCode::Enter => {
            filter_result.clear();
            for (i, note) in notes.iter().enumerate() {
                if note.text.contains(query.as_str()) || note.body.contains(query.as_str()) {
                    filter_result.push(ListItem::new(format!(
                        "{}: \"{}\" - \"{}\"",
                        i + 1,
                        note.text,
                        note.body
                    )));
                }
            }
            query.clear();
        }
        KeyCode::Backspace => {
            query.pop();
        }
        KeyCode::Esc => {
            *filter_popup_active = false;
            filter_result.clear();
            query.clear();
        }
        KeyCode::Char(c) => {
            query.push(c);
        }
        _ => {}
    }
}
