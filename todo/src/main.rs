mod list;
mod todo;

use clap::{Parser, Subcommand};
use list::{TodoError, TodoList};

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
            Some(todo) => Ok(vec![todo.format()]),
            None => Err(TodoError::NotFound(id)),
        },
        Command::List => {
            let mut lines = list.list();
            let open = list.count_open();
            lines.push(format!("未完了: {open} 件"));
            Ok(lines)
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let mut list = TodoList::new();
    match run(&mut list, cli.command) {
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
