use leptos::prelude::*;

struct Feature {
    icon: &'static str,
    title: &'static str,
    desc: &'static str,
}

const FEATURES: &[Feature] = &[
    Feature { icon: "🎯", title: "Tweens", desc: "Smooth value interpolation from A to B with builder pattern, delays, time scaling, and callbacks." },
    Feature { icon: "📐", title: "35+ Easing Curves", desc: "From Quad to Elastic to Bounce — plus CSS cubic-bezier(), Steps, RoughEase, SlowMo, and Wiggle." },
    Feature { icon: "🎹", title: "Keyframe Tracks", desc: "Multi-stop keyframe animations with per-segment easing. Supports Loop::Forever and PingPong." },
    Feature { icon: "🎬", title: "Timelines & Sequences", desc: "Compose animations concurrently or sequentially. GSAP-style stagger, At::Label positioning." },
    Feature { icon: "🌊", title: "Spring Physics", desc: "Damped harmonic oscillators with presets: gentle, wobbly, stiff, slow. Multi-dimensional SpringN." },
    Feature { icon: "✏️", title: "SVG Draw", desc: "Animate stroke-dashoffset for path drawing effects. draw_on() and draw_on_reverse() helpers." },
    Feature { icon: "🔮", title: "Shape Morph", desc: "Smooth interpolation between 2D point sets. Auto-resampling for mismatched point counts." },
    Feature { icon: "📜", title: "Scroll-Linked", desc: "ScrollDriver maps scroll position to animation progress. GSAP-style scroll-triggered animations." },
    Feature { icon: "🛤️", title: "Motion Paths", desc: "Bezier paths, Catmull-Rom splines, SVG path parsing. Animate elements along complex curves." },
    Feature { icon: "🎨", title: "Color Interpolation", desc: "Perceptually uniform color blending in Lab, Linear RGB, and OKLCh color spaces via palette." },
    Feature { icon: "🖱️", title: "Drag & Inertia", desc: "Drag constraints, axis locking, pointer data. Inertia physics for momentum-based interactions." },
    Feature { icon: "⚡", title: "WASM + Bevy", desc: "First-class requestAnimationFrame driver. Built-in Bevy 0.13 plugin. Runs everywhere Rust does." },
];

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <section class="section" id="features">
            <div class="reveal">
                <span class="section-label">"Capabilities"</span>
                <h2 class="section-title">"Everything you need to animate."</h2>
                <p class="section-desc">
                    "A complete animation toolkit — from simple tweens to physics simulations, all in pure Rust."
                </p>
            </div>

            <div class="features-grid">
                {FEATURES.iter().enumerate().map(|(i, feat)| {
                    view! {
                        <div
                            class="feature-card reveal"
                            style:transition-delay=format!("{}ms", i * 60)
                        >
                            <div class="feature-icon">{feat.icon}</div>
                            <h3>{feat.title}</h3>
                            <p>{feat.desc}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}
