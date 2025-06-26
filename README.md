# Note - zsh

Simple note-taking command-line tool written in Zsh.

## Installation

### 1. Clone the repository

**HTTPS**

```sh
git clone --depth=1 https://github.com/K10-K10/Note-zsh.git
```

**SSH**

```sh
git clone --depth=1 git@github.com:K10-K10/Note-zsh.git
```

### 2. Add to your PATH

Edit your `~/.zshrc`:

```sh
find ~ -name Note-zsh  # Check the path

echo 'export PATH="<path>:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Done!

---

## Usage

```sh
note add <Title> <Note body>  # Add a note
note list                     # List all notes
note del <number>            # Delete a note
note find <keyword>          # Search notes
note edit <number> -t "new"  # Edit title
note edit <number> -b "body" # Edit body
note help                    # Show help
```

---

## Commands

| Command                        | Description                                               | Options                                |
| ------------------------------ | --------------------------------------------------------- | -------------------------------------- |
| `note list`                    | List all saved notes                                      | `<Title>` – filter by title            |
| `note add <Title> <Note body>` | Add a new note (body is optional)                         |                                        |
| `note del <number>`            | Delete a note by number                                   |                                        |
| `note del`                     | Delete **all** notes (with confirmation)                  |                                        |
| `note find <keyword>`          | Search for a keyword (case-insensitive, highlights match) | `-t`, `-b` – search in title/body only |
| `note edit <number>`           | Edit an existing note                                     | `-t <new title>`, `-b <new body>`      |
| `note help`                    | Display usage help                                        |                                        |

---

## Demo

```sh
$ note add test "this is a test"
Note: Added "test" - "this is a test"

$ note list
Note:
0: test - this is a test

$ note find test
Note:
0: test - this is a **test**

$ note del 0
Note: Deleted note number 0
```

---

## License

This project is licensed under the [MIT License](LICENSE).
