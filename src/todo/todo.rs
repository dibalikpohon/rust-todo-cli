pub struct Todo {
    content: String,
    done: bool,
}

impl Todo {
    pub fn new(content: String, done: bool) -> Todo {
        Todo { content, done }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn exchange_content(&mut self, content: String) {
        self.content = content;
    }
}
