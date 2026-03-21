use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <header>
            <nav>
                <A href="/">
                    <strong>"Home"</strong>
                </A>
                " - "
                <A href="/users/foo-bar">
                    <strong>"Foo Bar"</strong>
                </A>
                " - "
                <A href="/users/baz">
                    <strong>"Baz"</strong>
                </A>
            </nav>
        </header>
    }
}
