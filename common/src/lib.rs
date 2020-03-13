pub mod models {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    pub type TodoId = i64;
    pub type Username = String;

    #[derive(Serialize, Deserialize)]
    pub struct TodoItem {
        pub id: i64,
        pub title: String,
        pub completed: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct User {
        pub username: String,
        pub todos_map: HashMap<TodoId, TodoItem>,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
