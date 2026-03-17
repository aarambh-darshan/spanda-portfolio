use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="section" id="about">
            <div class="reveal">
                <span class="section-label">"What is Spanda?"</span>
                <h2 class="section-title">
                    "The pulse of motion,"<br />
                    "written in Rust."
                </h2>
                <p class="section-desc">
                    "Spanda (Sanskrit: स्पन्द — vibration, pulse) is a general-purpose animation library for Rust. Zero mandatory dependencies, "
                    <code>"no_std"</code>"-ready, and designed to work anywhere: terminal UIs, web (WASM), game engines (Bevy), or native desktop apps."
                </p>

                <div class="stats-row">
                    <div class="stat-card">
                        <div class="stat-value">"35+"</div>
                        <div class="stat-label">"Easing Curves"</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">"0"</div>
                        <div class="stat-label">"Unsafe Code"</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">"6"</div>
                        <div class="stat-label">"Integration Targets"</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">"v0.8"</div>
                        <div class="stat-label">"Latest Release"</div>
                    </div>
                </div>

                <div class="about-badges">
                    <span class="about-badge">"🔧 no_std"</span>
                    <span class="about-badge">"🌐 WASM"</span>
                    <span class="about-badge">"🎮 Bevy"</span>
                    <span class="about-badge">"⚡ Tokio"</span>
                    <span class="about-badge">"📦 Serde"</span>
                    <span class="about-badge">"🎨 Palette"</span>
                </div>
            </div>
        </section>
    }
}
