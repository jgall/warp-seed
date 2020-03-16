use common::models::AuthHeader;
use common::models::RegisterUser;
use common::models::{TodoItem, User, Username};
use common::JsonReply;

use std::collections::HashMap;
use std::convert::Infallible;

use auth::PasswordHash;
use database::Db;
use warp::{http::StatusCode, path, reject::Rejection, Filter, Reply};

#[tokio::main]
async fn main() {
    let db = database::blank_db();

    let register_user = path("register")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(register_user);

    // GET /api/user => 200 OK with body "{User Json}"
    let get_user = warp::path!("user")
        .and(authenticate(db.clone()))
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(get_user);

    let add_todo = warp::path!("user")
        .and(authenticate(db.clone()))
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

async fn get_user(username: String, db: Db) -> Result<impl Reply, Infallible> {
    if let Some(user) = db.lock().await.get(&username) {
        Ok(user.user.json())
    } else {
        unimplemented!()
    }
}

async fn register_user(db: Db, user: RegisterUser) -> Result<impl Reply, Infallible> {
    if let Ok(hash) = PasswordHash::from_password(user.password) {
        let mut db = db.lock().await;
        if db.get(&user.username).is_some() {
            Ok(StatusCode::UNAUTHORIZED)
        } else {
            db.insert(
                user.username.clone(),
                database::DbUser {
                    username: user.username.clone(),
                    password: hash,
                    user: User {
                        username: user.username,
                        todos_map: HashMap::new(),
                    },
                },
            );
            Ok(StatusCode::OK)
        }
    } else {
        Ok(StatusCode::UNAUTHORIZED)
    }
}

async fn add_todo(username: String, todo: TodoItem, db: Db) -> Result<impl Reply, Infallible> {
    if let Some(user) = db.lock().await.get_mut(&username) {
        user.user.todos_map.insert(todo.id, todo);
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::UNAUTHORIZED)
    }
}

fn authenticate(db: Db) -> impl Filter<Extract = (Username,), Error = Rejection> + Clone {
    warp::any()
        .and(with_db(db))
        .and(warp::header::<AuthHeader>("authentication"))
        .and_then(|db: Db, auth: AuthHeader| async move {
            if let Some(user) = db.lock().await.get(&auth.username) {
                if user.password.verify(&auth.password) {
                    return Ok(auth.username);
                }
            }
            Err(warp::reject::not_found())
        })
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

mod database {
    use crate::auth::PasswordHash;
    use common::models::User;
    use common::models::Username;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type Db = Arc<Mutex<HashMap<Username, DbUser>>>;

    pub fn blank_db() -> Db {
        Arc::new(Mutex::new(HashMap::new()))
    }

    pub struct DbUser {
        pub username: String,
        pub password: PasswordHash,
        pub user: User,
    }
}

mod auth {
    use bcrypt::{hash, DEFAULT_COST};

    pub struct PasswordHash(String);
    impl PasswordHash {
        pub fn from_password(password: String) -> Result<Self, bcrypt::BcryptError> {
            Ok(PasswordHash(hash(password, DEFAULT_COST)?))
        }
        pub fn verify(&self, password: &str) -> bool {
            bcrypt::verify(password, &self.0).unwrap_or(false)
        }
    }
}
