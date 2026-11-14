use crate::list::TodoError;
use crate::sqlite_store::SqliteStore;
use crate::store::Store;
use crate::todo::TodoRow;
use askama::Template;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{
    Html, IntoResponse, Redirect, Response,
};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

struct AppState {
    store: Mutex<SqliteStore>,
}

fn api_error(error: TodoError) -> (StatusCode, String) {
    let status = match &error {
        TodoError::TooLong => StatusCode::BAD_REQUEST,
        TodoError::NotFound(_) => StatusCode::NOT_FOUND,
        TodoError::File(_) => {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    };
    (status, error.message())
}

async fn hello() -> String {
    "こんにちは".to_string()
}

async fn list_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TodoRow>>, (StatusCode, String)> {
    let store = state.store.lock().unwrap();
    let list = store.load().map_err(api_error)?;
    Ok(Json(list.to_rows()))
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(new): Form<NewTodo>,
) -> Result<String, (StatusCode, String)> {
    let store = state.store.lock().unwrap();
    let mut list = store.load().map_err(api_error)?;
    let id = list.add(&new.title).map_err(api_error)?;
    store.save(&list).map_err(api_error)?;
    Ok(format!("追加: {id} 番"))
}

async fn done_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<String, (StatusCode, String)> {
    let store = state.store.lock().unwrap();
    let mut list = store.load().map_err(api_error)?;
    list.get_mut(id).map_err(api_error)?.complete();
    store.save(&list).map_err(api_error)?;
    Ok(format!("完了: {id} 番"))
}

async fn remove_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<String, (StatusCode, String)> {
    let store = state.store.lock().unwrap();
    let mut list = store.load().map_err(api_error)?;
    list.remove(id).map_err(api_error)?;
    store.save(&list).map_err(api_error)?;
    Ok(format!("削除: {id} 番"))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {
    todos: Vec<TodoRow>,
    error: String,
}

fn render_page(
    store: &SqliteStore,
    error: String,
) -> Result<Html<String>, (StatusCode, String)> {
    let list = store.load().map_err(api_error)?;
    let page = IndexPage {
        todos: list.to_rows(),
        error,
    };
    let html = page.render().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )
    })?;
    Ok(Html(html))
}

async fn index(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let store = state.store.lock().unwrap();
    render_page(&store, String::new())
}

async fn add_page(
    State(state): State<Arc<AppState>>,
    Form(new): Form<NewTodo>,
) -> Result<Response, (StatusCode, String)> {
    let store = state.store.lock().unwrap();
    let mut list = store.load().map_err(api_error)?;
    match list.add(&new.title) {
        Ok(_) => {
            store.save(&list).map_err(api_error)?;
            Ok(Redirect::to("/").into_response())
        }
        Err(error) => {
            let page =
                render_page(&store, error.message())?;
            Ok((StatusCode::BAD_REQUEST, page)
                .into_response())
        }
    }
}

async fn done_page(
    state: State<Arc<AppState>>,
    id: Path<u32>,
) -> Result<Redirect, (StatusCode, String)> {
    done_todo(state, id).await?;
    Ok(Redirect::to("/"))
}

async fn remove_page(
    state: State<Arc<AppState>>,
    id: Path<u32>,
) -> Result<Redirect, (StatusCode, String)> {
    remove_todo(state, id).await?;
    Ok(Redirect::to("/"))
}

pub async fn serve(store: SqliteStore) {
    let state = Arc::new(AppState {
        store: Mutex::new(store),
    });
    let app = Router::new()
        .route("/", get(index))
        .route("/add", post(add_page))
        .route("/done/{id}", post(done_page))
        .route("/remove/{id}", post(remove_page))
        .route("/hello", get(hello))
        .route("/todos", get(list_todos).post(add_todo))
        .route("/todos/{id}/done", post(done_todo))
        .route("/todos/{id}", delete(remove_todo))
        .with_state(state);
    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .expect("3000 番のポートを使えない");
    println!("http://127.0.0.1:3000 で待っています");
    axum::serve(listener, app)
        .await
        .expect("サーバーが止まった");
}
