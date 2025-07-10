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
    filter_result: &mut Vec<ListItem<'a>>,
    query: &mut String,
    filter_popup_active: &mut bool,
    action: &mut bool,
) {
    *action = true;
    let area = note_title_input(60, f.area());
    let block = Block::default()
        .title("Filter notes")
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);
    let para = Paragraph::new(Text::from(query.clone())).block(block);
    f.render_widget(para, area);

    match key.code {
        //TODO: Some dellay is happened when second cher input
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
            *action = false;
            query.clear();
        }
        KeyCode::Backspace => {
            query.pop();
        }
        KeyCode::Esc => {
            *filter_popup_active = false;
            filter_result.clear();
            query.clear();
            *action = false;
        }
        KeyCode::Char(c) => {
            query.push(c);
        }
        _ => {}
    }
}
