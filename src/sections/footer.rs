use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer" id="footer">
            <div class="footer-grid">
                <div>
                    <div class="footer-brand">"spanda"</div>
                    <p class="footer-desc">
                        "A general-purpose animation library for Rust. Tweening, keyframes, timelines, springs & physics — anywhere Rust runs."
                    </p>
                    <div style="margin-top: 1rem;">
                        <code style="font-size: 0.8rem; color: rgba(255,255,255,0.3); background: rgba(255,255,255,0.04); padding: 0.4rem 0.8rem; border-radius: 6px;">
                            "cargo add spanda"
                        </code>
                    </div>
                </div>

                <div class="footer-links">
                    <h4>"Resources"</h4>
                    <a href="https://docs.rs/spanda" target="_blank">"Documentation"</a>
                    <a href="https://crates.io/crates/spanda" target="_blank">"crates.io"</a>
                    <a href="https://github.com/aarambh-darshan/spanda-docs" target="_blank">"Spanda Docs Site"</a>
                </div>

                <div class="footer-links">
                    <h4>"Community"</h4>
                    <a href="https://github.com/aarambh-darshan/spanda" target="_blank">"GitHub"</a>
                    <a href="https://github.com/aarambh-darshan/spanda/issues" target="_blank">"Report Issues"</a>
                    <a href="https://github.com/aarambh-darshan/spanda/blob/main/LICENSE-MIT" target="_blank">"MIT License"</a>
                </div>
            </div>

            <div class="footer-bottom">
                <span>"© 2025 Aarambh Darshan. MIT / Apache-2.0"</span>
                <span style="display: flex; align-items: center; gap: 0.4rem;">
                    "Built with "
                    <span style="color: #ff6b35; font-weight: 600;">"spanda"</span>
                    " + "
                    <span style="color: #e5c07b; font-weight: 600;">"Leptos"</span>
                    " 🦀"
                </span>
            </div>
        </footer>
    }
}
