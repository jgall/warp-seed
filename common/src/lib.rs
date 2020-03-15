use serde::{Deserialize, Serialize};
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
}

pub trait JsonReply
where
    Self: std::marker::Sized + Serialize,
{
    fn json(&self) -> warp::reply::Response {
        warp::reply::json(self).into_response()
    }
}

impl JsonReply for models::User {}

impl JsonReply for models::TodoItem {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
