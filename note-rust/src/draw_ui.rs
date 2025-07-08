use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};
pub fn draw_main_ui(
    f: &mut Frame,
    items: &Vec<ListItem>,
    list_state: &mut ListState,
    cmd_text: &str,
    filter_query: &str,
) {
    let size = f.area();

    let list_block_area = Rect::new(0, 0, size.width, size.height - 3);
    let cmd_block_area = Rect::new(0, size.height - 3, size.width, 3);

    let list = List::new(items.clone())
        .block(
            Block::default()
                .title("[1]: Notes")
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .highlight_symbol(">> ")
        .highlight_style(Style::default().bg(Color::Blue));

    f.render_stateful_widget(list, list_block_area, list_state);
    let mut cmd_display = cmd_text.to_string();
    cmd_display.push_str(&format!(" | Filtering: \"{}\"", filter_query));

    let cmd_block = Block::default()
        .title("")
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let cmd_paragraph = Paragraph::new(Text::from(cmd_display)).block(cmd_block.clone());
    f.render_widget(cmd_paragraph, cmd_block_area);
}
