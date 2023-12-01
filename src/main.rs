mod todo;
mod todo_command;

use core::fmt;
use std::{
    env::{self},
    process,
};

use todo::todo_list::TodoList;
use todo_command::TodoCommand;

use todo::todo::Todo;

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_done() {
            f.write_fmt(format_args!("[DONE] {}", self.get_content()))
        } else {
            f.write_fmt(format_args!("{}", self.get_content()))
        }
    }
}

fn main() {
    // Get arguments passed
    let args: Vec<String> = env::args().collect();

    let command = TodoCommand::new(&args);

    if let Err(why) = command {
        eprintln!("Error! {why}");
        process::exit(1);
    }

    match command.unwrap() {
        TodoCommand::New { list, file } => {
            // Create new TodoList
            let mut todo = TodoList::new();

            // append each list
            for l in list {
                todo.append(l);
            }

            // if file does not provided, defaults to "todo.txt"
            let file = file.unwrap_or("todo.txt".to_owned());
            todo.write_to_file(&file).expect("Cannot write to file!");
        }
        TodoCommand::Append { list, file } => {
            let file = file.unwrap_or("todo.txt".to_owned());
            let mut todo = TodoList::new_from_file(&file).expect("Cannot open file!");

            for l in list {
                todo.append(l);
            }

            // if file does not provided, defaults to "todo.txt"
            todo.write_to_file(&file).expect("Cannot write to file!");
        }
        TodoCommand::Delete { mut list, file } => {
            let file = file.unwrap_or("todo.txt".to_owned());
            let mut todo = TodoList::new_from_file(&file).expect("Cannot open file!");

            list.sort();
            list.dedup();

            // todos removed by list
            for i in 0..list.len() {
                let delete_index = list[i] as usize - i;
                if delete_index + 1 > todo.len() {
                    println!("Cannot delete at index {delete_index}. Out of bounds. Skipping...");
                    continue;
                }
                todo.delete(delete_index);
            }

            todo.write_to_file(&file).expect("Cannot write to file!");
        }
        TodoCommand::Done { mut list, file } => {
            let file = file.unwrap_or("todo.txt".to_owned());
            let mut todo = TodoList::new_from_file(&file).expect("Cannot open file!");

            list.sort();
            list.dedup();

            for l in list {
                if let Err(_) = todo.mark_done(l) {
                    println!("Index {l} does not exist. Skipping...");
                }
            }

            todo.write_to_file(&file).expect("Cannot write to file!");
        }
        TodoCommand::Edit {
            index,
            string,
            file,
        } => {
            let file = file.unwrap_or("todo.txt".to_owned());
            let mut todo = TodoList::new_from_file(&file).expect("Cannot open file!");

            if let Err(_) = todo.edit(index, string) {
                println!("Index {index} does not exist. Skipping...");

                // Does not yield error, but program should return 0
                process::exit(0)
            }

            todo.write_to_file(&file).expect("Cannot write to file!");
        }
        TodoCommand::Clear { file } => {
            let file = file.unwrap_or("todo.txt".to_owned());
            let mut todo = TodoList::new_from_file(&file).expect("Cannot open file!");

            todo.clear();

            todo.write_to_file(&file).expect("Cannot write to file!");
        }
        TodoCommand::Read { file, include_done } => {
            let file = file.unwrap_or("todo.txt".to_owned());
            let todo = TodoList::new_from_file(&file).expect("Cannot open file!");

            if todo.is_empty() {
                println!("== Todo is empty ==");
            } else {
                for (index, todo) in todo.iter().enumerate() {
                    if todo.is_done() {
                        if include_done {
                            println!("{index})\t{todo}");
                        }
                    } else {
                        println!("{index})\t{todo}");
                    }
                }
            }
        }
    }
}
