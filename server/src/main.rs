use common::models::{User, Username};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{
    http::StatusCode,
    path,
    reject::{reject, Rejection},
    reply::json,
    Filter, Reply,
};

#[tokio::main]
async fn main() {
    let db = blank_db();

    // GET /api/user/:username => 200 OK with body "{User Json}"
    let get_user = warp::path!("user" / String)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(get_user);

    let create_user = path("create")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(create_user);

    let api = path("api").and(get_user.or(create_user));

    let index = warp::any().map(|| "Hello, welcome to the index page :)");

    let routes = api.or(index);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn get_user(username: String, db: Db) -> Result<impl Reply, Rejection> {
    if let Some(user) = db.lock().await.get(&username) {
        Ok(json(user))
    } else {
        Err(not_found())
    }
}

async fn create_user(db: Db, user: User) -> Result<impl warp::Reply, Infallible> {
    let mut db = db.lock().await;
    if db.get(&user.username).is_some() {
        Ok(StatusCode::UNAUTHORIZED)
    } else {
        db.insert(user.username.clone(), user);
        Ok(StatusCode::OK)
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
