// use crate::cursor::Cursor;
use crate::sections::{about, code, demos, features, footer, get_started, hero, nav, splash};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        // <Cursor />
        <splash::Splash />
        <nav::Nav />
        <main>
            <hero::Hero />
            <about::About />
            <features::Features />
            <demos::Demos />
            <code::CodeExamples />
            <get_started::GetStarted />
            <footer::Footer />
        </main>
    }
}
