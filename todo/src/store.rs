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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load() {
        let path = std::env::temp_dir()
            .join("todo-store-test.json");
        let store =
            JsonStore::new(path.to_str().unwrap());
        let mut list = TodoList::new();
        list.add("牛乳を買う").unwrap();
        store.save(&list).unwrap();
        let mut loaded = store.load().unwrap();
        assert_eq!(loaded.list(), list.list());
        assert_eq!(loaded.add("卵を買う").unwrap(), 2);
        std::fs::remove_file(&path).unwrap();
    }
}
