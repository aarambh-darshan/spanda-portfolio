use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Nav() -> impl IntoView {
    let (scrolled, set_scrolled) = signal(false);

    Effect::new(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let doc_el = document.document_element().unwrap();

        let handle_scroll = move |_| {
            if doc_el.scroll_top() > 50 {
                set_scrolled.set(true);
            } else {
                set_scrolled.set(false);
            }
        };

        // Need to add event listener to window
        let closure = wasm_bindgen::closure::Closure::wrap(
            Box::new(handle_scroll) as Box<dyn FnMut(web_sys::Event)>
        );
        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    });

    view! {
        <nav
            class=move || format!(
                "fixed top-0 left-0 w-full z-50 flex items-center justify-between px-8 py-4 transition-colors duration-300 font-mono uppercase text-sm {}",
                if scrolled.get() {
                    "bg-bg border-b border-border"
                } else {
                    "bg-transparent border-transparent"
                }
            )
        >
            <div class="font-medium text-lg text-ink tracking-widest">
                <a href="#hero">"Spanda"</a>
            </div>

            <div class="flex gap-8 text-ink">
                <a href="#features" class="hover:text-accent-dark transition-colors">"Features"</a>
                <a href="#demos" class="hover:text-accent-dark transition-colors">"Interactive"</a>
                <a href="#code" class="hover:text-accent-dark transition-colors">"API"</a>
                <a href="#get-started" class="hover:text-accent-dark transition-colors">"Quick Start"</a>
            </div>

            <div>
                <a href="https://github.com/aarambh-darshan/spanda" target="_blank" class="hover:text-accent-dark transition-colors">
                    "GitHub ↗"
                </a>
            </div>
        </nav>
    }
}
