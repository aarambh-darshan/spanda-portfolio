use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-bg text-ink py-24 px-[5%] border-t border-border" id="footer">
            <div class="max-w-[1300px] mx-auto grid grid-cols-1 md:grid-cols-4 gap-12">
                <div class="md:col-span-2">
                    <div class="font-mono text-3xl font-bold tracking-widest uppercase mb-4 text-ink">"spanda"</div>
                    <p class="font-syne text-ink-muted text-base max-w-[400px] leading-relaxed">
                        "A general-purpose animation library for Rust. Tweening, keyframes, timelines, springs & physics — anywhere Rust runs."
                    </p>
                    <div class="mt-6">
                        <code class="font-code text-sm text-ink bg-black/5 px-4 py-2">
                            "cargo add spanda"
                        </code>
                    </div>
                </div>

                <div class="flex flex-col gap-4">
                    <h4 class="font-mono text-sm tracking-widest text-ink font-bold uppercase mb-2">"Resources"</h4>
                    <a href="https://docs.rs/spanda" target="_blank" class="font-syne text-ink-muted hover:text-accent-dark transition-colors">"Documentation"</a>
                    <a href="https://crates.io/crates/spanda" target="_blank" class="font-syne text-ink-muted hover:text-accent-dark transition-colors">"crates.io"</a>
                    <a href="https://github.com/aarambh-darshan/spanda-docs" target="_blank" class="font-syne text-ink-muted hover:text-accent-dark transition-colors">"Spanda Docs Site"</a>
                </div>

                <div class="flex flex-col gap-4">
                    <h4 class="font-mono text-sm tracking-widest text-ink font-bold uppercase mb-2">"Community"</h4>
                    <a href="https://github.com/aarambh-darshan/spanda" target="_blank" class="font-syne text-ink-muted hover:text-accent-dark transition-colors">"GitHub"</a>
                    <a href="https://github.com/aarambh-darshan/spanda/issues" target="_blank" class="font-syne text-ink-muted hover:text-accent-dark transition-colors">"Report Issues"</a>
                    <a href="https://github.com/aarambh-darshan/spanda/blob/main/LICENSE-MIT" target="_blank" class="font-syne text-ink-muted hover:text-accent-dark transition-colors">"MIT License"</a>
                </div>
            </div>

            <div class="max-w-[1300px] mx-auto mt-24 pt-8 border-t border-border flex flex-col md:flex-row justify-between items-center gap-4">
                <span class="font-mono text-xs text-ink-muted tracking-widest">"© 2025 Aarambh Darshan. MIT / Apache-2.0"</span>
                <span class="font-mono text-xs text-ink-muted flex items-center gap-2 tracking-widest">
                    "Built with "
                    <span class="font-bold text-accent-dark">"spanda"</span>
                    " + "
                    <span class="font-bold text-ink">"Leptos"</span>
                    " 🦀"
                </span>
            </div>
        </footer>
    }
}
