use crate::list::{TodoError, TodoList, file_error};

pub trait Store {
    fn load(&self) -> Result<TodoList, TodoError>;
    fn save(
        &self,
        list: &TodoList,
    ) -> Result<(), TodoError>;
}

pub struct JsonStore {
    path: String,
}

impl JsonStore {
    pub fn new(path: &str) -> JsonStore {
        JsonStore {
            path: path.to_string(),
        }
    }
}

impl Store for JsonStore {
    fn load(&self) -> Result<TodoList, TodoError> {
        if !std::path::Path::new(&self.path).exists() {
            return Ok(TodoList::new());
        }
        let text = std::fs::read_to_string(&self.path)
            .map_err(|e| file_error(e.to_string()))?;
        serde_json::from_str(&text)
            .map_err(|e| file_error(e.to_string()))
    }

    fn save(
        &self,
        list: &TodoList,
    ) -> Result<(), TodoError> {
        let text = serde_json::to_string_pretty(list)
            .expect("JSON に変換できない");
        std::fs::write(&self.path, text)
            .map_err(|e| file_error(e.to_string()))
    }
}
