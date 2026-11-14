fn format_todo(
    id: u32,
    title: String,
    done: bool,
) -> String {
    let mark = if done { "x" } else { " " };
    format!("[{mark}] {id}: {title}")
}

fn add_urgent(title: String) -> String {
    format!("{title}（急ぎ）")
}

fn main() {
    let id = 1;
    let title = String::from("牛乳を買う");
    let done = false;

    let line = format_todo(id, title.clone(), done);
    println!("{line}");
    println!("題名: {title}");
    println!("番号: {id}");

    let title = add_urgent(title);
    println!("{}", format_todo(id, title, done));
}
