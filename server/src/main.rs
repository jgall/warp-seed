use common::models::{TodoItem, User, Username};
use common::JsonReply;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{
    http::StatusCode,
    path,
    reject::{reject, Rejection},
    Filter, Reply,
};

#[tokio::main]
async fn main() {
    let db = blank_db();

    let register_user = path("register")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(register_user);

    // GET /api/user => 200 OK with body "{User Json}"
    let get_user = warp::path!("user")
        .and(warp::header("authorization"))
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(get_user);

    let add_todo = warp::path!("user")
        .and(warp::header("authorization"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(add_todo);

    let api = get_user.or(register_user).or(add_todo);
    let api = path("api").and(api);

    let index = warp::any().map(|| "Hello, welcome to the index page :)");

    let routes = api.or(index);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn get_user(username: String, db: Db) -> Result<impl Reply, Rejection> {
    if let Some(user) = db.lock().await.get(&username) {
        Ok(user.json())
    } else {
        Err(not_found())
    }
}

async fn register_user(db: Db, user: User) -> Result<impl warp::Reply, Infallible> {
    let mut db = db.lock().await;
    if db.get(&user.username).is_some() {
        Ok(StatusCode::UNAUTHORIZED)
    } else {
        db.insert(user.username.clone(), user);
        Ok(StatusCode::OK)
    }
}

async fn add_todo(
    username: String,
    todo: TodoItem,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    if let Some(user) = db.lock().await.get_mut(&username) {
        user.todos_map.insert(todo.id, todo);
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::UNAUTHORIZED)
    }
}

fn not_found() -> Rejection {
    reject()
}

#[derive(Debug)]
struct NotFound;

impl warp::reject::Reject for NotFound {}

type Db = Arc<Mutex<HashMap<Username, User>>>;

fn blank_db() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
