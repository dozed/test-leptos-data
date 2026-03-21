use itertools::Itertools;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{A, Route, Router, Routes},
    hooks::use_params_map,
    ParamSegment, StaticSegment,
};

use crate::{
    books::{Book, Person},
    data::load_person,
    nav::Nav,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Nav />
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=(StaticSegment("users"), ParamSegment("key")) view=UserPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! { <div>HomePage</div> }
}

#[component]
fn UserPage() -> impl IntoView {
    let params = use_params_map();
    // let key = params.read().get("key").unwrap_or_default();
    // let person = load_person(key);
    // let (person, _) = signal(person);

    let person = Resource::new(
        move || params.read().get("key").unwrap_or_default(),
        move |key| async move { load_person(key) },
    );

    view! {
        <div>
            <Suspense fallback=|| {
                view! { "Loading..." }
            }>
                {move || Suspend::new(async move {
                    match person.await {
                        Some(person) => {
                            // let (person, _) = signal(person);
                            view! { <BookList person=person /> }.into_any()
                        },
                        None => view! { <div>Not found</div> }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn BookList(person: Person) -> impl IntoView {
    let mut grouped = Vec::new();
    for (key, chunk) in &person.books.iter().rev().chunk_by(|b| b.year.clone()) {
        grouped.push((key, chunk.collect_vec()));
    }

    view! {
        <ul>
            {grouped
                .into_iter()
                .map(|(year, books)| {
                    view! {
                        <li class="year">{year}</li>
                        {books
                            .into_iter()
                            .map(|book| {
                                view! {
                                    <li class="entry">
                                        <BookItem person=&person book=book />
                                    </li>
                                }
                            })
                            .collect_view()}
                    }
                })
                .collect_view()}
        </ul>
    }
}

#[component]
fn BookItem<'a>(person: &'a Person, book: &'a Book) -> impl IntoView {
    view! {
        <div>
            <div>Title: {book.title.clone()}</div>
            <div>
                Authors:
                {book
                    .authors
                    .iter()
                    .enumerate()
                    .map(|(i, author)| {
                        let author_name = author.name.clone();

                        view! {
                            {(i > 0).then(|| ", ")}
                            {if author_name == person.name {
                                view! { <span style="text-decoration: underline">{author_name}</span> }.into_any()
                            } else {
                                view! {
                                    <span>
                                        <A href=format!("/users/{key}", key = author.key)>{author_name}</A>
                                    </span>
                                }
                                    .into_any()
                            }}
                        }
                    })
                    .collect_view()}
            </div>
            <div>Publisher: {book.publisher.clone()}</div>
        </div>
    }
}
