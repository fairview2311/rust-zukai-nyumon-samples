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

    fn is_open(&self) -> bool {
        matches!(self.status, Status::Open)
    }
}

struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn add(
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

    fn list(&self) -> Vec<String> {
        self.todos
            .iter()
            .map(|todo| todo.format())
            .collect()
    }

    fn find(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    fn find_mut(
        &mut self,
        id: u32,
    ) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    fn get_mut(
        &mut self,
        id: u32,
    ) -> Result<&mut Todo, TodoError> {
        match self.find_mut(id) {
            Some(todo) => Ok(todo),
            None => Err(TodoError::NotFound(id)),
        }
    }

    fn remove(
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

    fn count_open(&self) -> usize {
        self.todos
            .iter()
            .filter(|todo| todo.is_open())
            .count()
    }
}

fn clean_title(title: &str) -> String {
    title.trim().to_string()
}

fn is_too_long(title: &str) -> bool {
    title.chars().count() > 20
}

enum TodoError {
    TooLong,
    NotFound(u32),
    BadNumber(String),
    UnknownCommand(String),
}

impl TodoError {
    fn message(&self) -> String {
        match self {
            TodoError::TooLong => {
                "題名は20文字までです".to_string()
            }
            TodoError::NotFound(id) => {
                format!("{id} 番はありません")
            }
            TodoError::BadNumber(text) => {
                format!("番号ではありません: {text}")
            }
            TodoError::UnknownCommand(cmd) => {
                format!("知らないコマンド: {cmd}")
            }
        }
    }
}

fn parse_id(text: &str) -> Result<u32, TodoError> {
    match text.trim().parse() {
        Ok(id) => Ok(id),
        Err(_) => {
            Err(TodoError::BadNumber(text.to_string()))
        }
    }
}

fn split_arg(text: &str) -> (&str, &str) {
    match text.split_once(' ') {
        Some((head, rest)) => (head, rest),
        None => (text, ""),
    }
}

fn run(
    list: &mut TodoList,
    line: &str,
) -> Result<Vec<String>, TodoError> {
    let (cmd, arg) = split_arg(line);
    match cmd {
        "add" => {
            let id = list.add(arg)?;
            Ok(vec![format!("追加: {id} 番")])
        }
        "done" => {
            let id = parse_id(arg)?;
            list.get_mut(id)?.complete();
            Ok(vec![format!("完了: {id} 番")])
        }
        "later" => {
            let id = parse_id(arg)?;
            list.get_mut(id)?.postpone();
            Ok(vec![format!("あとで: {id} 番")])
        }
        "memo" => {
            let (id_text, memo) = split_arg(arg);
            let id = parse_id(id_text)?;
            list.get_mut(id)?.set_memo(memo);
            Ok(vec![format!("メモ: {id} 番")])
        }
        "remove" => {
            let id = parse_id(arg)?;
            list.remove(id)?;
            Ok(vec![format!("削除: {id} 番")])
        }
        "show" => {
            let id = parse_id(arg)?;
            match list.find(id) {
                Some(todo) => Ok(vec![todo.format()]),
                None => Err(TodoError::NotFound(id)),
            }
        }
        "list" => {
            let mut lines = list.list();
            let open = list.count_open();
            lines.push(format!("未完了: {open} 件"));
            Ok(lines)
        }
        _ => Err(TodoError::UnknownCommand(
            cmd.to_string(),
        )),
    }
}

fn main() {
    let mut list = TodoList::new();
    println!("add 題名 / done 番号 / later 番号");
    println!("memo 番号 メモ / remove 番号");
    println!("show 番号 / list / quit");
    loop {
        let mut input = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut input)
            .expect("入力を読み込めませんでした");
        let line = input.trim();
        if bytes == 0 || line == "quit" {
            break;
        }
        match run(&mut list, line) {
            Ok(lines) => {
                for text in lines {
                    println!("{text}");
                }
            }
            Err(error) => {
                println!("エラー: {}", error.message())
            }
        }
    }
}
