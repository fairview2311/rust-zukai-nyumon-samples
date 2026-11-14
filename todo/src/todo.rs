use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Open,
    Done,
    Later,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    title: String,
    status: Status,
    memo: Option<String>,
}

impl Todo {
    pub fn new(id: u32, title: &str) -> Todo {
        Todo {
            id,
            title: clean_title(title),
            status: Status::Open,
            memo: None,
        }
    }

    pub fn format(&self) -> String {
        let mark = match self.status {
            Status::Open => " ",
            Status::Done => "x",
            Status::Later => "-",
        };
        let id = self.id;
        let title = &self.title;
        let line = format!("[{mark}] {id}: {title}");
        if let Some(memo) = &self.memo {
            format!("{line}（メモ: {memo}）")
        } else {
            line
        }
    }

    pub fn complete(&mut self) {
        self.status = Status::Done;
    }

    pub fn postpone(&mut self) {
        self.status = Status::Later;
    }

    pub fn set_memo(&mut self, memo: &str) {
        self.memo = Some(memo.to_string());
    }

    pub fn has_word(&self, word: &str) -> bool {
        self.title.contains(word)
    }

    pub fn is_open(&self) -> bool {
        matches!(self.status, Status::Open)
    }
}

fn clean_title(title: &str) -> String {
    title.trim().to_string()
}

pub fn is_too_long(title: &str) -> bool {
    title.chars().count() > 20
}

impl fmt::Display for Todo {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

pub struct TodoRow {
    pub id: u32,
    pub title: String,
    pub status: String,
    pub memo: Option<String>,
}

impl Todo {
    pub fn to_row(&self) -> TodoRow {
        let status = match self.status {
            Status::Open => "open",
            Status::Done => "done",
            Status::Later => "later",
        };
        TodoRow {
            id: self.id,
            title: self.title.clone(),
            status: status.to_string(),
            memo: self.memo.clone(),
        }
    }

    pub fn from_row(row: TodoRow) -> Todo {
        let status = match row.status.as_str() {
            "done" => Status::Done,
            "later" => Status::Later,
            _ => Status::Open,
        };
        Todo {
            id: row.id,
            title: row.title,
            status,
            memo: row.memo,
        }
    }
}
