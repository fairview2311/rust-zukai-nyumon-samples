fn format_todo(
    id: u32,
    title: String,
    done: bool,
) -> String {
    let mark = if done { "x" } else { " " };
    format!("[{mark}] {id}: {title}")
}

fn main() {
    let id = 1;
    let title = String::from("牛乳を買う");
    let done = false;

    let line = format_todo(id, title, done);
    println!("{line}");
}
