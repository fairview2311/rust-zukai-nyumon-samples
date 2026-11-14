mod list;
mod store;
mod todo;

use clap::{Parser, Subcommand};
use list::{TodoError, TodoList};
use store::{JsonStore, Store};

#[derive(Parser)]
#[command(about = "ToDo を管理するコマンド")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// ToDo を追加する
    Add { title: String },
    /// ToDo を完了にする
    Done { id: u32 },
    /// ToDo をあとでにする
    Later { id: u32 },
    /// ToDo にメモを付ける
    Memo { id: u32, memo: String },
    /// ToDo を削除する
    Remove { id: u32 },
    /// ToDo を1つ表示する
    Show { id: u32 },
    /// ToDo の一覧を表示する
    List,
    /// 題名に語を含む ToDo を表示する
    Search { word: String },
}

fn run(
    list: &mut TodoList,
    command: Command,
) -> Result<Vec<String>, TodoError> {
    match command {
        Command::Add { title } => {
            let id = list.add(&title)?;
            Ok(vec![format!("追加: {id} 番")])
        }
        Command::Done { id } => {
            list.get_mut(id)?.complete();
            Ok(vec![format!("完了: {id} 番")])
        }
        Command::Later { id } => {
            list.get_mut(id)?.postpone();
            Ok(vec![format!("あとで: {id} 番")])
        }
        Command::Memo { id, memo } => {
            list.get_mut(id)?.set_memo(&memo);
            Ok(vec![format!("メモ: {id} 番")])
        }
        Command::Remove { id } => {
            list.remove(id)?;
            Ok(vec![format!("削除: {id} 番")])
        }
        Command::Show { id } => match list.find(id) {
            Some(todo) => Ok(vec![todo.to_string()]),
            None => Err(TodoError::NotFound(id)),
        },
        Command::Search { word } => {
            let found = list.search(&word);
            let mut lines: Vec<String> = found
                .iter()
                .map(|todo| todo.format())
                .collect();
            let count = found.len();
            lines.push(format!("該当: {count} 件"));
            Ok(lines)
        }
        Command::List => {
            let mut lines = list.list();
            let open = list.count_open();
            lines.push(format!("未完了: {open} 件"));
            Ok(lines)
        }
    }
}

fn run_and_save<S: Store>(
    store: &S,
    command: Command,
) -> Result<Vec<String>, TodoError> {
    let mut list = store.load()?;
    let lines = run(&mut list, command)?;
    store.save(&list)?;
    Ok(lines)
}

fn main() {
    let cli = Cli::parse();
    let store = JsonStore::new("todos.json");
    match run_and_save(&store, cli.command) {
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
