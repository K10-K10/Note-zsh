use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use ratatui::prelude::*;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub static VERSION_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let home = env::var("HOME").unwrap();
    PathBuf::from(home).join(".note-zsh/version.yml")
});
pub static FILE_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env::var("HOME").unwrap()).join(".note-zsh/note.txt"));

#[derive(Clone, Default)]
pub struct NoteFormat {
    pub text: String,
    pub body: String,
}

pub fn load_notes() -> Result<Vec<String>> {
    let file = File::open(&*FILE_PATH).or_else(|_| File::create(&*FILE_PATH))?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok).collect())
}

pub fn append_note_to_file(note: &str, body: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&*FILE_PATH)
        .or_else(|_| File::create(&*FILE_PATH))?;
    writeln!(file, "{:<100}", note)?;
    writeln!(file, "{:<100}", body)?;
    Ok(())
}

pub fn info_command() -> Result<String> {
    let file = match File::open(&*VERSION_PATH) {
        Ok(f) => f,
        Err(_) => return Ok("version: unknown\ncreated by: unknown".to_string()),
    };
    let reader = BufReader::new(file);
    let mut version = String::from("unknown");
    let mut created = String::from("unknown");
    for line in reader.lines().flatten() {
        if line.starts_with("version:") {
            version = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
        }
        if line.starts_with("created by:") {
            created = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
        }
    }
    Ok(format!("version: {}\ncreated by: {}", version, created))
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
    let mut file = File::create(&*FILE_PATH).or_else(|_| File::create(&*FILE_PATH))?;
    for note in notes {
        writeln!(file, "{:<100}", note.text)?;
        writeln!(file, "{:<100}", note.body)?;
    }
    Ok(())
}
