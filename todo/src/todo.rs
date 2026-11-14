#[derive(Debug)]
enum Status {
    Open,
    Done,
    Later,
}

#[derive(Debug)]
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
