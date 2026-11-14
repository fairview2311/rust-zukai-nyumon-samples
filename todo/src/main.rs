fn format_todo(
    id: u32,
    title: &String,
    done: bool,
) -> String {
    let mark = if done { "x" } else { " " };
    format!("[{mark}] {id}: {title}")
}

fn complete(done: &mut bool) {
    *done = true;
}

fn main() {
    let id = 1;
    let title = String::from("牛乳を買う");
    let mut done = false;

    println!("{}", format_todo(id, &title, done));
    println!("題名: {title}");

    complete(&mut done);
    println!("{}", format_todo(id, &title, done));
}
