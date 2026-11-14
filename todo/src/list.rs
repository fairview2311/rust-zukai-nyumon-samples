use crate::todo::{Todo, TodoRow, is_too_long};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
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

pub fn file_error(text: String) -> TodoError {
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

impl TodoList {
    pub fn to_rows(&self) -> Vec<TodoRow> {
        self.todos
            .iter()
            .map(|todo| todo.to_row())
            .collect()
    }

    pub fn from_rows(rows: Vec<TodoRow>) -> TodoList {
        let next_id = rows
            .iter()
            .map(|row| row.id)
            .max()
            .unwrap_or(0)
            + 1;
        let todos = rows
            .into_iter()
            .map(Todo::from_row)
            .collect();
        TodoList { todos, next_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_new_id() {
        let mut list = TodoList::new();
        assert_eq!(list.add("牛乳を買う").unwrap(), 1);
        assert_eq!(list.add("卵を買う").unwrap(), 2);
    }

    #[test]
    fn add_rejects_long_title() {
        let mut list = TodoList::new();
        let title = "あ".repeat(21);
        let result = list.add(&title);
        assert!(matches!(
            result,
            Err(TodoError::TooLong)
        ));
    }

    #[test]
    fn done_reduces_open_count() {
        let mut list = TodoList::new();
        list.add("牛乳を買う").unwrap();
        list.add("卵を買う").unwrap();
        list.get_mut(1).unwrap().complete();
        assert_eq!(list.count_open(), 1);
    }

    #[test]
    fn todo_is_shown_with_display() {
        let mut list = TodoList::new();
        list.add("牛乳を買う").unwrap();
        let todo = list.find(1).unwrap();
        assert_eq!(
            todo.to_string(),
            "[ ] 1: 牛乳を買う"
        );
    }
}
