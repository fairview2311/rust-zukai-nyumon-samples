#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

impl Todo {
    fn new(id: u32, title: &str) -> Todo {
        Todo {
            id,
            title: clean_title(title),
            done: false,
        }
    }

    fn format(&self) -> String {
        let mark = if self.done { "x" } else { " " };
        format!("[{mark}] {}: {}", self.id, self.title)
    }

    fn complete(&mut self) {
        self.done = true;
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
    let egg = Todo::new(2, "卵を買う");

    if is_too_long(&todo.title) {
        println!("題名は20文字までです");
    }
    println!("{}", todo.format());

    todo.complete();
    println!("{}", todo.format());
    println!("{}", egg.format());
    println!("{todo:?}");
}
