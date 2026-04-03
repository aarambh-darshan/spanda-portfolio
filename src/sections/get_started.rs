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
            <section class="bg-code-bg text-white py-32 px-[5%]" id="get-started">
                <div class="reveal max-w-[1000px] mx-auto text-center pt-16">
                    <span class="font-mono text-accent text-sm tracking-widest uppercase font-bold">"// QUICK START"</span>
                    <h2 class="font-instrument italic text-[4rem] lg:text-[6rem] leading-none mt-6 text-white pt-6">
                        "Start animating "
                        <span class="text-accent">"in seconds."</span>
                    </h2>
                    <p class="font-syne text-white/60 text-lg lg:text-xl mt-8 max-w-[700px] mx-auto leading-relaxed">
                        "Add Spanda to your Rust project and bring your UI to life with physics-based, production-ready animations."
                    </p>

                    // Install command
                    <div class="bg-bg border-l-[4px] border-accent mt-16 max-w-[800px] mx-auto text-left p-6 flex flex-col md:flex-row items-start md:items-center justify-between gap-6 relative">
                        <div>
                            <div class="font-mono text-xs text-ink/40 tracking-widest uppercase mb-4">"Terminal"</div>
                            <code class="font-code text-xl text-ink">
                                <span class="text-ink/40">"$ "</span>
                                "cargo add spanda"
                            </code>
                        </div>
                        <button
                            class="font-mono uppercase text-sm border-2 border-ink text-ink bg-transparent px-8 py-3 hover:bg-accent hover:border-accent transition-colors rounded-none"
                            on:click=copy_cmd
                        >
                            {move || if copied.get() { "✓ Copied" } else { "⎘ Copy" }}
                        </button>
                    </div>

                    // Quick example
                    <div class="bg-[#111] border-l-[4px] border-accent mt-8 max-w-[800px] mx-auto text-left relative">
                        <div class="flex items-center gap-2 px-6 py-4 border-b border-white/10 bg-white/5">
                            <div class="w-2.5 h-2.5 rounded-full bg-[#ff5f57]" />
                            <div class="w-2.5 h-2.5 rounded-full bg-[#febc2e]" />
                            <div class="w-2.5 h-2.5 rounded-full bg-[#28c840]" />
                            <span class="ml-3 font-mono text-xs text-white/40 tracking-widest uppercase">"main.rs"</span>
                        </div>
                        <pre class="p-8 font-code text-sm leading-relaxed overflow-x-auto text-white">
    <span class="text-accent">"use "</span><span class="text-accent">"spanda"</span><span class="text-white">"::{Tween, Easing};\n"</span>
    <span class="text-accent">"use "</span><span class="text-accent">"spanda"</span><span class="text-white">"::traits::Update;\n\n"</span>
    <span class="text-accent">"let mut "</span><span class="text-white">"tween = "</span><span class="text-accent">"Tween"</span><span class="text-white">"::new("</span><span class="text-white">"0.0_f32"</span><span class="text-white">", "</span><span class="text-white">"100.0"</span><span class="text-white">")\n"</span>
    <span class="text-white">"    .duration("</span><span class="text-white">"1.0"</span><span class="text-white">")\n"</span>
    <span class="text-white">"    .easing("</span><span class="text-accent">"Easing"</span><span class="text-white">"::EaseOutExpo)\n"</span>
    <span class="text-white">"    .build();\n\n"</span>
    <span class="text-white/40">"// In your game/UI loop:\n"</span>
    <span class="text-white">"tween.update(dt);\n"</span>
    <span class="text-accent">"let "</span><span class="text-white">"value = tween.value(); "</span><span class="text-white/40">"// 0.0 → 100.0"</span>
                        </pre>
                    </div>

                    // Links row
                    <div class="flex gap-6 justify-center mt-16 flex-wrap">
                        <a href="https://docs.rs/spanda" target="_blank" class="font-mono uppercase text-sm border-2 border-white px-8 py-4 text-white hover:bg-accent hover:text-black hover:border-accent transition-colors rounded-none">
                            "📖 API Docs"
                        </a>
                        <a href="https://github.com/aarambh-darshan/spanda-docs" target="_blank" class="font-mono uppercase text-sm border-2 border-white px-8 py-4 text-white hover:bg-accent hover:text-black hover:border-accent transition-colors rounded-none">
                            "📚 Guide"
                        </a>
                        <a href="https://crates.io/crates/spanda" target="_blank" class="font-mono uppercase text-sm border-2 border-white px-8 py-4 text-white hover:bg-accent hover:text-black hover:border-accent transition-colors rounded-none">
                            "📦 crates.io"
                        </a>
                        <a href="https://github.com/aarambh-darshan/spanda" target="_blank" class="font-mono uppercase text-sm border-2 border-white px-8 py-4 text-white hover:bg-accent hover:text-black hover:border-accent transition-colors rounded-none">
                            "⭐ Star on GitHub"
                        </a>
                    </div>

                    // Badges
                    <div class="flex gap-3 justify-center mt-12 flex-wrap">
                        <span class="font-mono text-xs uppercase text-white/50 border border-white/10 px-4 py-2 rounded-none hover:text-accent transition-colors">"MIT Licensed"</span>
                        <span class="font-mono text-xs uppercase text-white/50 border border-white/10 px-4 py-2 rounded-none hover:text-accent transition-colors">"no_std ready"</span>
                        <span class="font-mono text-xs uppercase text-white/50 border border-white/10 px-4 py-2 rounded-none hover:text-accent transition-colors">"WASM compatible"</span>
                        <span class="font-mono text-xs uppercase text-white/50 border border-white/10 px-4 py-2 rounded-none hover:text-accent transition-colors">"Bevy integration"</span>
                        <span class="font-mono text-xs uppercase text-white/50 border border-white/10 px-4 py-2 rounded-none hover:text-accent transition-colors">"zero unsafe"</span>
                    </div>
                </div>
            </section>
        }
}
