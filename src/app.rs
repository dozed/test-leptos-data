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
    let books = person.read().books.clone();

    let mut grouped: Vec<(String, Vec<Book>)> = Vec::new();
    for (key, chunk) in &books.into_iter().rev().chunk_by(|b| b.year.clone()) {
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
fn BookItem(person: ReadSignal<Person>, book: Book) -> impl IntoView {
    view! {
        <div>
            <div>Title: {book.title.clone()}</div>
            <div>Authors:
            {
                book.authors.into_iter().enumerate().map(|(i, author)| view! {
                    <Show when={move || i > 0}>
                        {", "}
                    </Show>
                    <Show when={
                        let author_name = author.name.clone();
                        move || person.read().name == author_name
                    } clone:author>
                        <span style="text-decoration: underline">
                            {author.name.clone()}
                        </span>
                    </Show>
                    <Show when={
                        let author_name = author.name.clone();
                        move || person.read().name != author_name
                    } clone:author>
                        <span>
                            {author.name.clone()}
                        </span>
                    </Show>
                }).collect_view()
            }
            </div>
        </div>
    }
}
