# 図解でわかるRust入門 サンプルコード

書籍『図解でわかるRust入門 所有権のしくみからWebアプリ開発まで』（早瀬 透）のサンプルコードです。

本では、ToDo アプリ「todo」を1冊を通して育てます。このリポジトリには、**各章を終えた時点の todo** が入っています。
本文のコードの枠と、その章のサンプルコードは同じ内容です。

- 書いたコードが動かないとき、本のとおりの完成形と見比べる
- 途中の章から読み始めるとき、前の章の終わりの状態をそろえる

といった使い方をしてください。本文の途中で試す例や、わざとエラーを出す例は入っていません。

## 本

- Kindle 版: https://www.amazon.co.jp/dp/B0HRCKR2T3
- ペーパーバック: https://www.amazon.co.jp/dp/B0HRFJK8ZH

## ファイルを入手する方法

入手する方法は3つあります。やりたいことに合わせて選んでください。

| やりたいこと | 方法 |
|---|---|
| 1つのファイルだけ見比べたい・コピーしたい | 下の表の［ファイルを見る］から、ブラウザーでファイルを開く。右上のコピーのボタンで中身をコピーできます |
| ある章の終わりの todo を全部手元に置きたい | 下の表の［ZIP］を押してダウンロードし、展開する |
| 全部の章を手元に置いて、章を切り替えたい | `git clone` でクローンし、`git checkout ch05` のように切り替える（下の「git でクローンする方法」） |

ZIP を展開したり git でクローンしたりしたら、中の `todo` のフォルダーを、VS Code の［ファイル］メニューの［フォルダーを開く...］（Mac は［開く...］）で開きます。本文で「タグ `ch10` の `todo/src/main.rs`」と書いた所は、下の表の第10章の行から開けます。

| 章 | この章の終わりの todo | ファイルを見る | ダウンロード |
|---|---|---|---|
| 第1章 | `cargo new todo` で作った、`Hello, world!` を表示するプロジェクト | [ch01](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch01/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch01.zip) |
| 第2章 | ToDo の題名を表示する `main.rs` と、`rustfmt.toml` | [ch02](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch02/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch02.zip) |
| 第3章 | 変数と型で ToDo 1件（番号・題名・完了）を表す | [ch03](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch03/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch03.zip) |
| 第4章 | `format_todo` で表示の形を整える関数 | [ch04](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch04/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch04.zip) |
| 第5章 | `clone` で題名を渡し、所有権のエラーを直した形 | [ch05](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch05/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch05.zip) |
| 第6章 | `&` と `&mut` で、持ち主を移さずに貸す形 | [ch06](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch06/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch06.zip) |
| 第7章 | `&str` で受け取り、`clean_title` で題名を整える | [ch07](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch07/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch07.zip) |
| 第8章 | `Todo` 構造体とメソッド | [ch08](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch08/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch08.zip) |
| 第9章 | `Status`（未完了・完了・あとで）と、メモの `Option` | [ch09](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch09/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch09.zip) |
| 第10章 | `Vec` で一覧を持つ `TodoList`（追加・一覧・完了・削除） | [ch10](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch10/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch10.zip) |
| 第11章 | `Result` と `?` で、キーボードから打つコマンドを読む | [ch11](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch11/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch11.zip) |
| 第12章 | `list.rs`・`todo.rs` に分け、clap で `cargo run -- add 牛乳を買う` の形に | [ch12](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch12/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch12.zip) |
| 第13章 | `todos.json` に保存して、次の起動でも読み込む | [ch13](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch13/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch13.zip) |
| 第14章 | 題名で探す `search`（ライフタイム） | [ch14](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch14/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch14.zip) |
| 第15章 | `Display` と、保存先の `Store` トレイト・`JsonStore` | [ch15](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch15/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch15.zip) |
| 第16章 | 単体テストと結合テスト（`tests/cli.rs`） | [ch16](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch16/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch16.zip) |
| 第17章 | SQLite（`todos.db`）に保存する `SqliteStore` | [ch17](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch17/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch17.zip) |
| 第18章 | `examples/` の、待ち時間を重ねる tokio の例 | [ch18](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch18/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch18.zip) |
| 第19章 | axum の Web API（`cargo run -- serve`） | [ch19](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch19/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch19.zip) |
| 第20章 | askama のテンプレートで作るブラウザーの画面 | [ch20](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch20/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch20.zip) |
| 第21章 | 環境変数で設定できる完成形（このページの `todo` と同じ） | [ch21](https://github.com/fairview2311/rust-zukai-nyumon-samples/tree/ch21/todo) | [ZIP](https://github.com/fairview2311/rust-zukai-nyumon-samples/archive/refs/tags/ch21.zip) |

### git でクローンする方法

Git をインストールしている人は、ターミナルで次のように打ちます。

```text
git clone https://github.com/fairview2311/rust-zukai-nyumon-samples.git
cd rust-zukai-nyumon-samples
git checkout ch05
```

`ch01`〜`ch21` は Git のタグ（ある時点のファイルに付けた名前）です。`git checkout ch05` で第5章の終わりの状態に、`git checkout main` で完成形に戻ります。自分で書き換えたファイルは、切り替える前に別の場所へコピーしておいてください。

## 動かし方

本の第1章の手順で Rust をインストールしてから、`todo` のフォルダーを VS Code で開き、［ターミナル］メニューの［新しいターミナル］で開いたターミナルで打ちます。

| 章 | 動かすコマンド |
|---|---|
| 第1〜11章 | `cargo run`（第11章は、起動してからコマンドを1行ずつ打ちます） |
| 第12〜17章 | `cargo run -- add 牛乳を買う`・`cargo run -- list` のように、`--` の後ろにサブコマンドを付けます |
| 第18章 | `cargo run --example async_wait`（`examples/` の例を動かします） |
| 第19〜21章 | `cargo run -- serve` を打ち、ブラウザーで http://127.0.0.1:3000/ を開きます（第19章は API なので curl で確かめます）。止めるときは Ctrl＋C |

テストは、第16章からどの章でも `cargo test` で動きます。

第12章から使うクレート（clap・serde・rusqlite・tokio・axum・askama）は、初めてビルドしたときに自動でダウンロードされます。インターネットにつながった状態で動かしてください。第17章の rusqlite は SQLite 本体もビルドするので、初回は数十秒から数分かかります。

## 完成形のフォルダーの中身（第21章）

```text
todo/
├── Cargo.toml          使うクレートと版
├── rustfmt.toml        コードの形を整える設定（1行56文字）
├── src/main.rs         コマンドの入り口（clap）
├── src/todo.rs         ToDo 1件の型（Todo・Status）
├── src/list.rs         一覧の型（TodoList）とエラーの型
├── src/store.rs        保存先の約束（Store）と JSON の保存先
├── src/sqlite_store.rs SQLite の保存先
├── src/web.rs          Web API と画面（axum）
├── templates/          画面のテンプレート（askama）
├── tests/cli.rs        コマンドを外から起動する結合テスト
└── examples/           第18章・第21章の、本筋とは別の例
```

## うまくいかないとき

- `rustc --version` が 1.99.1 より古い … `rustup update` で新しいバージョンにします。Homebrew で入れた Rust が先に見つかっている場合は、本の第2章「うまくいかないとき」を見ます
- Windows で `link.exe` が見つからない … 本の第1章の手順で C++ Build Tools をインストールします
- Windows で curl の結果がおかしい … PowerShell では `curl` ではなく `curl.exe` と打ちます（第19章）
- `serve` が「そのアドレスを使えない」で止まる … 別のターミナルでサーバーが起動したままになっています。Ctrl＋C で停止します（第19章・第21章）

## 確かめたバージョン

- Rust 1.99.1（2024 edition）
- clap 4.6.7・serde 1.0.229・serde_json 1.0.154・rusqlite 0.42.0・tokio 1.53.3・axum 0.8.9・askama 0.16.1
- Windows 11・macOS（2026 年 11 月）

## ライセンス

MIT License（[LICENSE](LICENSE) を見てください）。本のコードは、自分のアプリに自由に使ってかまいません。
