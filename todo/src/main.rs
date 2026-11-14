fn main() {
    let id: u32 = 1;
    let title = String::from("牛乳を買う");
    let mut done = false;
    println!("{id}: {title} 完了: {done}");

    done = true;
    println!("{id}: {title} 完了: {done}");
}
