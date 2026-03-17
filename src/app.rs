use leptos::prelude::*;
use crate::sections::{splash, hero, about, features, demos, code, get_started, footer};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <splash::Splash />
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
