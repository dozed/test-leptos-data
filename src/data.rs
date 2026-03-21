use crate::books::{Author, Book, Person};

pub fn load_person(key: String) -> Person {
    match key.as_ref() {
        "foo-bar" => Person {
            key: "foo-bar".to_string(),
            name: "Foo Bar".to_string(),
            books: vec![
                Book {
                    title: "Book 1".to_string(),
                    year: "2020".to_string(),
                    authors: vec![
                        Author {
                            name: "Foo Bar".to_string(),
                        },
                        Author {
                            name: "Baz".to_string(),
                        },
                    ],
                    publisher: "A".to_string(),
                },
                Book {
                    title: "Book 2".to_string(),
                    year: "2020".to_string(),
                    authors: vec![
                        Author {
                            name: "Peter Fnord".to_string(),
                        },
                        Author {
                            name: "Foo Bar".to_string(),
                        },
                    ],
                    publisher: "B".to_string(),
                },
                Book {
                    title: "Book 3".to_string(),
                    year: "2023".to_string(),
                    authors: vec![Author {
                        name: "Foo Bar".to_string(),
                    }],
                    publisher: "A".to_string(),
                },
            ],
        },
        "baz" => Person {
            key: "baz".to_string(),
            name: "Baz".to_string(),
            books: vec![
                Book {
                    title: "Book 4".to_string(),
                    year: "2018".to_string(),
                    authors: vec![Author {
                        name: "Baz".to_string(),
                    }],
                    publisher: "A".to_string(),
                },
                Book {
                    title: "Book 1".to_string(),
                    year: "2020".to_string(),
                    authors: vec![
                        Author {
                            name: "Foo Bar".to_string(),
                        },
                        Author {
                            name: "Baz".to_string(),
                        },
                    ],
                    publisher: "B".to_string(),
                },
            ],
        },
        _ => {
            panic!("invalid user")
        }
    }
}
