fn format_todo(
    id: u32,
    title: &str,
    done: bool,
) -> String {
    let mark = if done { "x" } else { " " };
    format!("[{mark}] {id}: {title}")
}

fn complete(done: &mut bool) {
    *done = true;
}

fn clean_title(title: &str) -> String {
    title.trim().to_string()
}

fn is_too_long(title: &str) -> bool {
    title.chars().count() > 20
}

fn main() {
    let id = 1;
    let input = String::from("  牛乳を買う  ");
    let title = clean_title(&input);
    let mut done = false;

    if is_too_long(&title) {
        println!("題名は20文字までです");
    }
    println!("{}", format_todo(id, &title, done));

    complete(&mut done);
    println!("{}", format_todo(id, &title, done));
    println!("{}", format_todo(2, "卵を買う", false));
}
