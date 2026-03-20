use itertools::Itertools;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::books::{Author, Book, Person};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (person, _) = signal(Person {
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
            },
            Book {
                title: "Book 3".to_string(),
                year: "2023".to_string(),
                authors: vec![Author {
                    name: "Foo Bar".to_string(),
                }],
            },
        ],
    });

    view! {
        <BookList person={person} />
    }
}

#[component]
fn BookList(person: ReadSignal<Person>) -> impl IntoView {
    let mut grouped: Vec<(String, Vec<&Book>)> = Vec::new();
    let person_value = person.read();
    for (key, chunk) in &person_value.books.iter().rev().chunk_by(|b| b.year.clone()) {
        grouped.push((key.clone(), chunk.collect()));
    }

    view! {
        <ul>
            {grouped.into_iter().map(|(year, books)| view! {
                <>
                    <li class="year">{year}</li>
                    {books.into_iter().map(|book| view! {
                        <li class="entry">
                            <BookItem person={person} book={book} />
                        </li>
                    }).collect_view()}
                </>
            }).collect_view()}
        </ul>
    }
}

#[component]
fn BookItem<'a>(person: ReadSignal<Person>, book: &'a Book) -> impl IntoView {
    view! {
        <div>
            <div>Title: {book.title.clone()}</div>
            <div>Authors:
            {
                book.authors.iter().enumerate().map(|(i, author)| {
                    let (author_name, _) = signal(author.name.clone());

                    view! {
                        {move || if i > 0 { ", " } else { "" }}
                        {
                            if author_name.read() == person.read().name {
                                view! {
                                    <span style="text-decoration: underline">
                                        {author_name}
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <span>
                                        {author_name}
                                    </span>
                                }.into_any()
                            }
                        }
                    }
                }).collect_view()
            }
            </div>
        </div>
    }
}
