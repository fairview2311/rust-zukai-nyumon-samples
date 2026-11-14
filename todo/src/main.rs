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

    fn add(&mut self, title: &str) -> u32 {
        let id = self.next_id;
        self.todos.push(Todo::new(id, title));
        self.next_id += 1;
        id
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

    fn done(&mut self, id: u32) -> bool {
        if let Some(todo) = self.find_mut(id) {
            todo.complete();
            true
        } else {
            false
        }
    }

    fn remove(&mut self, id: u32) -> bool {
        let before = self.todos.len();
        self.todos.retain(|todo| todo.id != id);
        self.todos.len() < before
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

fn main() {
    let mut list = TodoList::new();
    let title = "  牛乳を買う  ";
    if is_too_long(title) {
        println!("題名は20文字までです");
    }
    list.add(title);
    let egg = list.add("卵を買う");
    list.add("本を返す");
    list.add("手紙を出す");

    list.done(1);
    if let Some(todo) = list.find_mut(1) {
        todo.set_memo("2本");
    }
    if let Some(todo) = list.find_mut(3) {
        todo.postpone();
    }
    list.remove(egg);
    if !list.done(9) {
        println!("9 番の ToDo はありません");
    }
    if let Some(todo) = list.find(4) {
        println!("4 番: {}", todo.title);
    }

    for line in list.list() {
        println!("{line}");
    }
    println!("未完了: {} 件", list.count_open());
}
