# Note - zsh

Simple note-taking command line tool in Zsh + TUI written in Rust.

> \[!NOTE]
> CLI & TUI note-taking tool.

---

## Installation

### 1. Clone the repo

HTTPS

```sh
git clone --depth=1 https://github.com/K10-K10/note-zsh.git
```

SSH

```sh
git clone --depth=1 git@github.com:K10-K10/Note-zsh.git
```

### 2. Add to PATH

Edit your `~/.zshrc` to include the path:

```sh
find ~/ -name Note-zsh # check the path

echo 'export PATH="<path>:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

---

## CLI Usage (Zsh script)

Use it from anywhere:

```sh
note add <Title> <Note body>
note list
```

### Commands

| Command                                     | Description                                         | Option                                    |
| ------------------------------------------- | --------------------------------------------------- | ----------------------------------------- |
| `note list`                                 | List all saved notes                                | `<Title>` filter by title                 |
| `note add <Title> <Note body>`              | Add a new note. You can leave the note body empty.  |                                           |
| `note del <number>`                         | Delete note by number                               |                                           |
| `note del all`                              | Delete all notes (with confirmation)                |                                           |
| `note find <keyword>`                       | Search notes (case-insensitive, highlights matches) | `-t`, `-b` (Search only in title or body) |
| `note edit <number> <new title> <new body>` | Edit note                                           | `-t <new title>`, `-b <new body>`         |
| `note tui`                                  | Launch TUI interface                                |                                           |
| `note help`                                 | Show help message                                   |                                           |

---

## TUI

You can also run the interactive TUI interface with:

```sh
note tui
```

![CUI main](images/CUI-main.png)

Features:

* List and navigate notes
* Add, edit, delete notes interactively
* Filter/search notes
* Error messages and confirmation dialogs

---

## Demo

```sh
$ note add test test2
Note: Added "test" - "test2"

$ note add hoge
Note: Added "hoge" - ""

$ note list
Note:
0: test - test2
1: hoge -

$ note del 1
Note: Deleted note number 1

$ note find test
0: test - test2

$ note tui
# Launches the interactive UI
```

---

## Cargo (TUI)

To build TUI manually:

```sh
cd note-rust
cargo run --release
```

Make sure you have `Rust` installed:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## License

MIT
