use color_eyre::eyre::Result;
use ratatui::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub static FILE_PATH: &str = "note.txt";
static INFO_PATH: &str = "version.yml";

pub fn info_command() -> Result<String> {
    let file = File::open(INFO_PATH)?;
    let reader = BufReader::new(file);

    let mut version = String::new();
    let mut created = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("version:") {
            if let Some(value) = line.splitn(2, ':').nth(1) {
                version = value.trim().to_string();
            }
        }
        if line.starts_with("created by") {
            if let Some(value) = line.splitn(2, ':').nth(1) {
                created = value.trim_start_matches(':').trim().to_string();
            }
        }
    }

    if !created.is_empty() && !version.is_empty() {
        Ok(format!("{} : {}", created, version))
    } else {
        Ok("unknown".to_string())
    }
}

#[derive(Clone, Default)]
pub struct NoteFormat {
    pub text: String,
    pub body: String,
}

pub fn load_notes() -> Result<Vec<String>> {
    let file = File::open(FILE_PATH).unwrap_or_else(|_| File::create(FILE_PATH).unwrap());
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}
pub fn append_note_to_file(note: &str, body: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_PATH)?;
    writeln!(file, "{:<100}", note)?;
    writeln!(file, "{:<100}", body)?;
    Ok(())
}

pub fn note_title_input(percent_x: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 150;
    let popup_height = 3;
    let popup_x = r.x + (r.width - popup_width) / 2;
    let popup_y = r.y + (r.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}

pub fn note_body_input(percent_x: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 150;
    let popup_height = 9;
    let popup_x = r.x + (r.width - popup_width) / 2;
    let popup_y = r.y + (r.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}

pub fn save_notes(notes: &Vec<NoteFormat>) -> std::io::Result<()> {
    let mut file = std::fs::File::create(FILE_PATH)?;
    for note in notes {
        let text_line = format!("{:<100}\n", note.text);
        let body_line = format!("{:<100}\n", note.body);
        file.write_all(text_line.as_bytes())?;
        file.write_all(body_line.as_bytes())?;
    }
    Ok(())
}

// pub struct command_popup_active {
//     add: i8,
//     edit: i8,
//     error: i8,
//     filter: i8,
//     movemexnt: i8,
// }
