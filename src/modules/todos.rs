#[derive(Clone, Debug)]
pub struct Todo {
    pub content: String,
    pub id: u8
}

#[derive(Clone)]
pub struct Todos {
    pub todos: Vec<Todo>,
}

impl Todos {
    pub fn init() -> Self {
        Todos { todos: Vec::new() }
    }

    pub fn create_todo(&mut self, content: String) -> Todo {
        let new_todo = Todo {
            content,
            id: self.todos.len() as u8 + 1
        };
        
        self.todos.push(new_todo.clone());

        return new_todo;
    }

    pub fn delete_todo(&mut self, id: u8) {
        let index = self.todos.iter().position(|todo| todo.id == id).unwrap();
        self.todos.remove(index);
    }
}
