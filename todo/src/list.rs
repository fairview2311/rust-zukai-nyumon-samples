use crate::todo::{Todo, is_too_long};
use serde::{Deserialize, Serialize};

pub enum TodoError {
    TooLong,
    NotFound(u32),
    File(String),
}

impl TodoError {
    pub fn message(&self) -> String {
        match self {
            TodoError::TooLong => {
                "題名は20文字までです".to_string()
            }
            TodoError::NotFound(id) => {
                format!("{id} 番はありません")
            }
            TodoError::File(text) => {
                format!("ファイルを使えません: {text}")
            }
        }
    }
}

fn file_error(text: String) -> TodoError {
    TodoError::File(text)
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn load(
        path: &str,
    ) -> Result<TodoList, TodoError> {
        if !std::path::Path::new(path).exists() {
            return Ok(TodoList::new());
        }
        let text = std::fs::read_to_string(path)
            .map_err(|e| file_error(e.to_string()))?;
        serde_json::from_str(&text)
            .map_err(|e| file_error(e.to_string()))
    }

    pub fn save(
        &self,
        path: &str,
    ) -> Result<(), TodoError> {
        let text = serde_json::to_string_pretty(self)
            .expect("JSON に変換できない");
        std::fs::write(path, text)
            .map_err(|e| file_error(e.to_string()))
    }

    pub fn add(
        &mut self,
        title: &str,
    ) -> Result<u32, TodoError> {
        if is_too_long(title) {
            return Err(TodoError::TooLong);
        }
        let id = self.next_id;
        self.todos.push(Todo::new(id, title));
        self.next_id += 1;
        Ok(id)
    }

    pub fn list(&self) -> Vec<String> {
        self.todos
            .iter()
            .map(|todo| todo.format())
            .collect()
    }

    pub fn search(&self, word: &str) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| todo.has_word(word))
            .collect()
    }

    pub fn find(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    fn find_mut(
        &mut self,
        id: u32,
    ) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    pub fn get_mut(
        &mut self,
        id: u32,
    ) -> Result<&mut Todo, TodoError> {
        match self.find_mut(id) {
            Some(todo) => Ok(todo),
            None => Err(TodoError::NotFound(id)),
        }
    }

    pub fn remove(
        &mut self,
        id: u32,
    ) -> Result<(), TodoError> {
        let before = self.todos.len();
        self.todos.retain(|todo| todo.id != id);
        if self.todos.len() < before {
            Ok(())
        } else {
            Err(TodoError::NotFound(id))
        }
    }

    pub fn count_open(&self) -> usize {
        self.todos
            .iter()
            .filter(|todo| todo.is_open())
            .count()
    }
}
