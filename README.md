# Note - zsh  
Simple note-taking command line tool in Zsh.


---

## Installation

### 1.Clone the repo
HTTPS
```sh
git clone --depth=1 https://github.com/K10-K10/note-zsh.git
```
SSH
```sh
git clone --depth=1 git@github.com:K10-K10/Note-zsh.git
```
### 2.Add to Path
Edit your ~/.zshrc to include the path:
```sh
find ~/ -name  Note-zsh # check the path

echo 'export PATH="<path>:$PATH"' >> ~/.zshrc 
source ~/.zshrc
```

Done! 

## Use it from anywhere
```sh
note add <Title> <Note body>
note list
```

## Commands
| Command                        | Description                                                         | Option                           |
| ------------------------------ | ------------------------------------------------------------------- | -------------------------------- |
| note list                      | List all saved notes                                                |                                  |
| note add \<Title> \<Note body> | Add a new note. You can leave the note body empty.                  |                                  |
| note del \<number>             | Delete note by number                                               |                                  |
| note del                       | all	Delete all notes (with confirmation)                            |                                  |
| note find \<keyword>           | Search notes for the keyword (case-insensitive, highlights matches) | -t, -b (Search just in tab,body) |
| note help                      | Show help message                                                   |                                  |

## Demo
- Add Note
```sh
$ note add test test2
Note: Added "test" - "test2"
```

- You cna select the empty note body
```sh
$ note add hoge
Note: Added "hoge" - ""
```

- Show notes
```sh
$ note list
Note:
0: test - test2
1: hoge - 
```

- Delete note
```sh
$ note del 1
Note: Deleted note number 1
```

- Find keyword
```sh
$ note del test
7: test - test hoge
```
