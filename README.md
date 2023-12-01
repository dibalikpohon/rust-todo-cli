# todo-list

Creating todo list has never been easier with this command line app. Written in Rust.

It's simple! You just have to run this command

## Create new todo
```
cargo run -- new todo-1 todo-2 todo-3 -o <output_file>
```
This command creates todos and write to `<output_file>`,  
the `-o` tag is optional and defaults to `todo.txt`.

## Append new todo
```
cargo run -- append todo-4 todo-5 todo-6 -o <output_file>
```
This command appends existing todo list from `<output_file>`
and also writes to `<output_file>`.

## Read todo
```
cargo run -- read -o <output_file> -d
```
This command reads your todo file and ~uploads it to the Internet~
show it to the screen. The `-d` tag includes todo marked as done.
The `-o` does the same thing as the last two commands!

## Mark todo as done
```
cargo run -- done <index-1> <index-2>... -o <output_file>
```
This command marks some todos as done. Don't worry, we don't delete
it yet, if you wish to delete it, use the `delete` command (we'll explain it later).

## Delete your todo
```
cargo run -- delete <index-1> <index-2>... -o <output_file>
```
As explained before, this command deletes your todo. Warning! This does not prompt
anything if the deleted todo has not been marked as done. This feature will be implemented
when the pigs fly~ 🐖🪽


# End of word
This project made for fun! Should any bugs found in this project, please don't hesitate to 
*not* reporting them to me.
