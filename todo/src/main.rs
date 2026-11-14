#[derive(Debug)]
enum Status {
    Open,
    Done,
    Later,
}

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    status: Status,
    memo: Option<String>,
}

impl Todo {
    fn new(id: u32, title: &str) -> Todo {
        Todo {
            id,
            title: clean_title(title),
            status: Status::Open,
            memo: None,
        }
    }

    fn format(&self) -> String {
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

    fn complete(&mut self) {
        self.status = Status::Done;
    }

    fn postpone(&mut self) {
        self.status = Status::Later;
    }

    fn set_memo(&mut self, memo: &str) {
        self.memo = Some(memo.to_string());
    }
}

fn clean_title(title: &str) -> String {
    title.trim().to_string()
}

fn is_too_long(title: &str) -> bool {
    title.chars().count() > 20
}

fn main() {
    let mut todo = Todo::new(1, "  牛乳を買う  ");
    let mut egg = Todo::new(2, "卵を買う");
    let book = Todo::new(3, "本を返す");

    if is_too_long(&todo.title) {
        println!("題名は20文字までです");
    }
    todo.set_memo("2本");
    todo.complete();
    egg.postpone();

    println!("{}", todo.format());
    println!("{}", egg.format());
    println!("{}", book.format());
    println!("{:?}", book.memo);
    println!("{:?}", todo.memo);
}
