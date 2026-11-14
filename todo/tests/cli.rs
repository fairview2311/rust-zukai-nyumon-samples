use std::process::Command;

#[test]
fn add_prints_new_id() {
    let dir =
        std::env::temp_dir().join("todo-cli-test");
    std::fs::create_dir_all(&dir).unwrap();
    let json = dir.join("todos.json");
    if json.exists() {
        std::fs::remove_file(&json).unwrap();
    }
    let output =
        Command::new(env!("CARGO_BIN_EXE_todo"))
            .args(["add", "牛乳を買う"])
            .current_dir(&dir)
            .output()
            .unwrap();
    assert!(output.status.success());
    let text =
        String::from_utf8(output.stdout).unwrap();
    assert_eq!(text, "追加: 1 番\n");
}
