use serde::Serialize;
use warp::reply::Reply;

pub mod models {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    pub type TodoId = i64;
    pub type Username = String;

    #[derive(Serialize, Deserialize)]
    pub struct TodoItem {
        pub id: TodoId,
        pub title: String,
        pub completed: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct User {
        pub username: String,
        pub todos_map: HashMap<TodoId, TodoItem>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AuthHeader {
        pub username: String,
        pub password: String,
    }

    impl std::str::FromStr for AuthHeader {
        type Err = serde_json::Error;
        fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
            serde_json::from_str(s)
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct RegisterUser {
        pub username: String,
        pub password: String,
    }
}

pub trait JsonReply
where
    Self: std::marker::Sized + Serialize,
{
    fn json(&self) -> warp::reply::Response {
        warp::reply::json(self).into_response()
    }
}

impl<T> JsonReply for T where Self: std::marker::Sized + Serialize {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
