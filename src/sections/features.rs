use leptos::prelude::*;

struct Feature {
    icon: &'static str,
    title: &'static str,
    desc: &'static str,
}

const FEATURES: &[Feature] = &[
    Feature { icon: "🎯", title: "Tweens", desc: "Smooth value interpolation from A to B with builder pattern, delays, time scaling, and callbacks." },
    Feature { icon: "🚀", title: "GPU Compute", desc: "Batch evaluate thousands of tweens concurrently using WGSL compute shaders. Includes auto-CPU fallback." },
    Feature { icon: "🛠️", title: "Layout Anim", desc: "Auto-generate FLIP sequence animations for DOM elements. Cross-view shared element transitions." },
    Feature { icon: "📱", title: "Gestures", desc: "Platform-agnostic gesture recognition for Tap, Swipe, Pinch, and Rotate with configurable thresholds." },
    Feature { icon: "📐", title: "35+ Easing Curves", desc: "From Quad to Elastic to Bounce — plus CSS cubic-bezier(), Steps, RoughEase, SlowMo, and Wiggle." },
    Feature { icon: "🎹", title: "Keyframe Tracks", desc: "Multi-stop keyframe animations with per-segment easing. Supports Loop::Forever and PingPong." },
    Feature { icon: "🎬", title: "Timelines & Sequences", desc: "Compose animations concurrently or sequentially. GSAP-style stagger, At::Label positioning." },
    Feature { icon: "🌊", title: "Spring Physics", desc: "Damped harmonic oscillators with presets: gentle, wobbly, stiff, slow. Multi-dimensional SpringN." },
    Feature { icon: "✏️", title: "SVG Draw", desc: "Animate stroke-dashoffset for path drawing effects. draw_on() and draw_on_reverse() helpers." },
    Feature { icon: "🔮", title: "Shape Morph", desc: "Smooth interpolation between 2D point sets. Auto-resampling and shape index optimization." },
    Feature { icon: "📜", title: "Scroll-Linked", desc: "ScrollDriver maps scroll position to animation progress with custom snap points and callbacks." },
    Feature { icon: "🛤️", title: "Motion Paths", desc: "Bezier paths, Catmull-Rom splines, SVG path parsing. Animate elements along complex curves." },
    Feature { icon: "🎨", title: "Color Interpolation", desc: "Perceptually uniform color blending via palette. Auto-detect and parse HEX/CSS named colors." },
    Feature { icon: "🖱️", title: "Drag & Inertia", desc: "Drag constraints, click threshold validation, pointer data. Inertia physics for momentum interaction." },
    Feature { icon: "⚡", title: "WASM + Bevy", desc: "First-class requestAnimationFrame driver. Built-in Bevy 0.13 plugin. Runs everywhere Rust does." },
];

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <section class="bg-bg text-ink py-32 px-[5%]" id="features">
            <div class="reveal max-w-[1300px] mx-auto pt-16">
                <span class="font-mono text-accent-dark text-sm tracking-widest uppercase font-bold">"// BUILT TO ANIMATE"</span>
                <h2 class="font-instrument text-[5rem] text-ink leading-none mt-6">"Everything you need to animate."</h2>
                <p class="font-syne text-ink-muted text-lg mt-6 max-w-[600px]">
                    "A complete animation toolkit — from simple tweens to physics simulations, all in pure Rust."
                </p>
            </div>

            <div class="max-w-[1300px] mx-auto mt-16">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {FEATURES.iter().enumerate().map(|(i, feat)| {
                        let num_str = format!("{:02}", i + 1);
                        view! {
                            <div class="reveal group border border-border p-8 hover:border-accent transition-colors duration-300">
                                <div class="flex items-start gap-4">
                                    <span class="font-instrument italic text-5xl text-outline leading-none shrink-0">
                                        {num_str}
                                    </span>
                                    <div>
                                        <h3 class="font-syne font-bold text-xl text-ink group-hover:text-accent-dark transition-colors">{feat.title}</h3>
                                        <p class="font-syne font-normal text-sm text-ink-muted mt-3 leading-relaxed">{feat.desc}</p>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
