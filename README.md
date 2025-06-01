# Note - zsh  
Simple note-taking command line tool in Zsh.


---

## Installation

### 1.Clone the repo
```sh
git clone https://github.com/yourname/note-zsh.git
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
| Command                        | Description                                                                        |
| ------------------------------ | ---------------------------------------------------------------------------------- |
| note list                      | List all saved notes                                                               |
| note add \<Title> \<Note body> | Add a new note.You can select the empty note body. It will be saved as empty note. |
| note del \<number>             | Delete note by number                                                              |
| note del all                   | Delete all notes (with confirm)                                                    |
| note help                      | Show help message                                                                  |

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
