use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn GetStarted() -> impl IntoView {
    let (copied, set_copied) = signal(false);

    let copy_cmd = move |_| {
        // Use JS to copy text to clipboard
        if let Some(window) = web_sys::window() {
            let _ = js_sys::eval("navigator.clipboard.writeText('cargo add spanda')");
            set_copied.set(true);
            let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                set_copied.set(false);
            });
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                2000,
            );
        }
    };

    view! {
        <section class="section" id="get-started">
            <div class="reveal" style="text-align: center; max-width: 800px; margin: 0 auto;">
                <span class="section-label">"Get Started"</span>
                <h2 class="section-title" style="text-align: center;">
                    "Start animating in seconds."
                </h2>
                <p class="section-desc" style="text-align: center; margin: 0 auto 3rem;">
                    "Add Spanda to your Rust project and bring your UI to life with physics-based, production-ready animations."
                </p>

                // Install command
                <div class="install-card">
                    <div class="install-header">
                        <span style="font-size: 0.7rem; color: rgba(255,255,255,0.35); font-family: 'JetBrains Mono', monospace;">"Terminal"</span>
                    </div>
                    <div class="install-body">
                        <code style="font-size: 1.1rem; color: #98c379;">
                            <span style="color: rgba(255,255,255,0.4);">"$ "</span>
                            "cargo add spanda"
                        </code>
                        <button
                            class="copy-btn"
                            on:click=copy_cmd
                        >
                            {move || if copied.get() { "✓ Copied" } else { "⎘ Copy" }}
                        </button>
                    </div>
                </div>

                // Quick example
                <div class="install-card" style="margin-top: 1.5rem;">
                    <div class="install-header">
                        <div style="display: flex; gap: 0.4rem; align-items: center;">
                            <div style="width: 8px; height: 8px; border-radius: 50%; background: #ff5f57;" />
                            <div style="width: 8px; height: 8px; border-radius: 50%; background: #febc2e;" />
                            <div style="width: 8px; height: 8px; border-radius: 50%; background: #28c840;" />
                        </div>
                        <span style="font-size: 0.7rem; color: rgba(255,255,255,0.35); font-family: 'JetBrains Mono', monospace;">"main.rs"</span>
                    </div>
                    <pre class="code-body" style="font-size: 0.85rem; line-height: 1.8;">
<span class="kw">"use "</span><span class="ty">"spanda"</span><span class="kw">"::{Tween, Easing};\n"</span>
<span class="kw">"use "</span><span class="ty">"spanda"</span><span class="kw">"::traits::Update;\n\n"</span>
<span class="kw">"let mut "</span><span>"tween = "</span><span class="ty">"Tween"</span><span>"::new("</span><span class="num">"0.0_f32"</span><span>", "</span><span class="num">"100.0"</span><span>")\n"</span>
<span>"    .duration("</span><span class="num">"1.0"</span><span>")\n"</span>
<span>"    .easing("</span><span class="ty">"Easing"</span><span>"::EaseOutExpo)\n"</span>
<span>"    .build();\n\n"</span>
<span class="cm">"// In your game/UI loop:\n"</span>
<span>"tween.update(dt);\n"</span>
<span class="kw">"let "</span><span>"value = tween.value(); "</span><span class="cm">"// 0.0 → 100.0"</span>
                    </pre>
                </div>

                // Links row
                <div style="display: flex; gap: 1.5rem; justify-content: center; margin-top: 3rem; flex-wrap: wrap;">
                    <a href="https://docs.rs/spanda" target="_blank" class="get-started-link">
                        <span>"📖"</span>
                        <span>"API Docs"</span>
                    </a>
                    <a href="https://github.com/aarambh-darshan/spanda-docs" target="_blank" class="get-started-link">
                        <span>"📚"</span>
                        <span>"Guide"</span>
                    </a>
                    <a href="https://crates.io/crates/spanda" target="_blank" class="get-started-link">
                        <span>"📦"</span>
                        <span>"crates.io"</span>
                    </a>
                    <a href="https://github.com/aarambh-darshan/spanda" target="_blank" class="get-started-link">
                        <span>"⭐"</span>
                        <span>"Star on GitHub"</span>
                    </a>
                </div>

                // Badges
                <div style="display: flex; gap: 0.75rem; justify-content: center; margin-top: 2rem; flex-wrap: wrap;">
                    <span class="about-badge">"MIT Licensed"</span>
                    <span class="about-badge">"no_std ready"</span>
                    <span class="about-badge">"WASM compatible"</span>
                    <span class="about-badge">"Bevy integration"</span>
                    <span class="about-badge">"zero unsafe"</span>
                </div>
            </div>
        </section>
    }
}
