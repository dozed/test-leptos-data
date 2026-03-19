
#[derive(Clone)]
pub struct Person {
    pub name: String,
    pub books: Vec<Book>,
}

#[derive(Clone)]
pub struct Book {
    pub title: String,
    pub year: String,
    pub authors: Vec<Author>,
}

#[derive(Clone)]
pub struct Author {
    pub name: String,
}
