use std::{fs::File, io::Write, ops::Add};

use crate::todo::todo::Todo;

pub struct TodoList(Vec<Todo>);

impl TodoList {
    pub fn new() -> TodoList {
        TodoList(Vec::new())
    }

    pub fn new_from_file(file_path: &String) -> Result<TodoList, String> {
        match std::fs::read_to_string(file_path) {
            Ok(contents) => Ok(TodoList(
                contents
                    .lines()
                    .map(|s| {
                        let done = s.strip_prefix("\x01");
                        let todo = done.unwrap_or(s);
                        Todo::new(todo.to_string(), done.is_some())
                    })
                    .collect::<Vec<Todo>>(),
            )),

            Err(e) => Err(e.to_string()),
        }
    }

    pub fn write_to_file(&self, file_path: &String) -> Result<(), String> {
        let ref list = self.0;
        let mut file_create = match File::create(file_path) {
            Err(why) => return Err(why.to_string()),
            Ok(file) => file,
        };
        for todo in list {
            if todo.is_done() {
                file_create.write_fmt(format_args!("{}", "\x01")).expect("Cannot write");
            }
            file_create.write_fmt(format_args!("{}\n", todo.get_content())).expect("Cannot write");
        }
        Ok(())
    }

    pub fn append(&mut self, str: String) -> &Todo {
        self.0.push(Todo::new(str, false));
        self.0.last().unwrap()
    }

    pub fn edit(&mut self, index: usize, str: String) -> Result<&Todo, String> {
        if let Some(todo) = self.0.get_mut(index) {
            todo.exchange_content(str);
            Ok(todo)
        } else {
            Err("Index out of bounds".to_owned())
        }
    }

    pub fn mark_done(&mut self, index: usize) -> Result<&Todo, String> {
        if let Some(todo) = self.0.get_mut(index) {
            todo.set_done(true);
            Ok(todo)
        } else {
            Err("Index out of bounds".to_owned())
        }
    }

    pub fn delete(&mut self, index: usize) -> Todo {
        self.0.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&Todo> {
        self.0.get(index)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> TodoIter {
        TodoIter { todos: self, current: 0 }
    }
}

pub struct TodoIter<'a> {
    todos: &'a TodoList,
    current: usize,
}

impl<'a> Iterator for TodoIter<'a> {
    type Item = &'a Todo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.todos.len() {
           let todo = self.todos.get(self.current); 
            self.current = self.current.add(1);
            todo
        } else {
            None
        }
    }
}
