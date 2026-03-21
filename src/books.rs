use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Person {
    pub key: String,
    pub name: String,
    pub books: Vec<Book>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub year: String,
    pub authors: Vec<Author>,
    pub publisher: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Author {
    pub key: String,
    pub name: String,
}
