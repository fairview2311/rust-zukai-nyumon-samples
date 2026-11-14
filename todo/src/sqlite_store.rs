use crate::list::{TodoError, TodoList, file_error};
use crate::store::Store;
use crate::todo::TodoRow;
use rusqlite::Connection;

const CREATE_TABLE: &str =
    "CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    memo TEXT
)";

const SELECT_ALL: &str =
    "SELECT id, title, status, memo
    FROM todos ORDER BY id";

const INSERT: &str =
    "INSERT INTO todos (id, title, status, memo)
    VALUES (?1, ?2, ?3, ?4)";

fn db_error(error: rusqlite::Error) -> TodoError {
    file_error(error.to_string())
}

pub struct SqliteStore {
    path: String,
}

impl SqliteStore {
    pub fn new(path: &str) -> SqliteStore {
        SqliteStore {
            path: path.to_string(),
        }
    }

    fn open(&self) -> Result<Connection, TodoError> {
        let conn = Connection::open(&self.path)
            .map_err(db_error)?;
        conn.execute(CREATE_TABLE, ())
            .map_err(db_error)?;
        Ok(conn)
    }
}

impl Store for SqliteStore {
    fn load(&self) -> Result<TodoList, TodoError> {
        let conn = self.open()?;
        let mut stmt = conn
            .prepare(SELECT_ALL)
            .map_err(db_error)?;
        let rows = stmt
            .query_map((), |row| {
                Ok(TodoRow {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    status: row.get(2)?,
                    memo: row.get(3)?,
                })
            })
            .map_err(db_error)?;
        let mut todo_rows = Vec::new();
        for row in rows {
            todo_rows.push(row.map_err(db_error)?);
        }
        Ok(TodoList::from_rows(todo_rows))
    }

    fn save(
        &self,
        list: &TodoList,
    ) -> Result<(), TodoError> {
        let mut conn = self.open()?;
        let tx =
            conn.transaction().map_err(db_error)?;
        tx.execute("DELETE FROM todos", ())
            .map_err(db_error)?;
        for row in list.to_rows() {
            let params = (
                row.id, row.title, row.status, row.memo,
            );
            tx.execute(INSERT, params)
                .map_err(db_error)?;
        }
        tx.commit().map_err(db_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load() {
        let path = std::env::temp_dir()
            .join("todo-sqlite-test.db");
        let store =
            SqliteStore::new(path.to_str().unwrap());
        let mut list = TodoList::new();
        list.add("牛乳を買う").unwrap();
        list.add("卵を買う").unwrap();
        list.get_mut(1).unwrap().complete();
        store.save(&list).unwrap();
        let mut loaded = store.load().unwrap();
        assert_eq!(loaded.list(), list.list());
        assert_eq!(
            loaded.add("パンを買う").unwrap(),
            3
        );
        std::fs::remove_file(&path).unwrap();
    }
}
