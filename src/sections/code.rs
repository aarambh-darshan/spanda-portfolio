use crate::animation;
use leptos::prelude::*;
use spanda::traits::Update as _;
use spanda::tween::TweenState;
use spanda::{Easing, Spring, SpringConfig, Tween};
use std::cell::RefCell;
use std::rc::Rc;

#[component]
pub fn CodeExamples() -> impl IntoView {
    view! {
        <section class="bg-bg text-ink py-32 px-[5%]" id="code">
            <div class="reveal max-w-[1300px] mx-auto pt-16">
                <span class="font-mono text-accent-dark text-sm tracking-widest uppercase font-bold">"// SIMPLE API"</span>
                <h2 class="font-instrument text-[4rem] text-ink leading-none mt-6">"Simple, expressive API."</h2>
                <p class="font-syne text-ink-muted text-lg mt-6 max-w-[600px] leading-relaxed">
                    "Add spanda to your Cargo.toml and start animating in seconds. Builder pattern, fluent API, zero boilerplate."
                </p>
            </div>

            // ── Example 1: Basic Tween ──
            <TweenExample />

            // ── Example 2: Spring ──
            <SpringExample />

            // ── Example 3: Timeline ──
            <TimelineExample />

            <GpuComputeExample />
        </section>
    }
}

#[component]
fn TweenExample() -> impl IntoView {
    let (value, set_value) = signal(0.0_f32);
    let (running, set_running) = signal(false);

    let play = move |_| {
        if running.get() {
            return;
        }
        set_running.set(true);
        set_value.set(0.0);

        let tween = Rc::new(RefCell::new(
            Tween::new(0.0_f32, 100.0)
                .duration(1.5)
                .easing(Easing::EaseOutCubic)
                .build(),
        ));

        let tc = tween.clone();
        animation::start_raf_loop(move |dt| {
            let mut t = tc.borrow_mut();
            if *t.state() != TweenState::Completed {
                t.update(dt);
                set_value.set(t.value());
            } else {
                set_running.set(false);
            }
        });
    };

    view! {
            <div class="flex flex-col lg:flex-row gap-12 mt-16 mb-24 max-w-[1300px] mx-auto reveal">
                <div class="flex-1 lg:sticky lg:top-[120px] h-fit">
                    <div class="bg-code-bg border-l-4 border-accent text-white font-mono rounded-none">
                        <div class="flex items-center gap-2 px-5 py-3 border-b border-white/10 bg-white/5">
                            <div class="w-2.5 h-2.5 rounded-full bg-[#ff5f57]" />
                            <div class="w-2.5 h-2.5 rounded-full bg-[#febc2e]" />
                            <div class="w-2.5 h-2.5 rounded-full bg-[#28c840]" />
                            <span class="ml-3 text-xs text-white/40 font-mono tracking-widest">"tween.rs"</span>
                        </div>
                    <pre class="p-6 text-sm leading-relaxed overflow-x-auto text-white">
    <span class="text-accent">"use "</span><span class="text-accent">"spanda"</span><span class="text-accent">"::{"</span><span class="text-accent">"Tween"</span><span class="text-white">", "</span><span class="text-accent">"Easing"</span><span class="text-white">"};\n"</span>
    <span class="text-accent">"use "</span><span class="text-accent">"spanda"</span><span class="text-white">"::traits::"</span><span class="text-accent">"Update"</span><span class="text-white">";\n\n"</span>
    <span class="text-accent">"let mut "</span><span class="text-white">"tween = "</span><span class="text-accent">"Tween"</span><span class="text-white">"::new("</span><span class="text-white">"0.0"</span><span class="text-white">", "</span><span class="text-white">"100.0"</span><span class="text-white">")\n"</span>
    <span class="text-white">"    .duration("</span><span class="text-white">"1.5"</span><span class="text-white">")\n"</span>
    <span class="text-white">"    .easing("</span><span class="text-accent">"Easing"</span><span class="text-white">"::EaseOutCubic)\n"</span>
    <span class="text-white">"    .build();\n\n"</span>
    <span class="text-white/40">"// Each frame:\n"</span>
    <span class="text-white">"tween.update(dt);\n"</span>
    <span class="text-accent">"let "</span><span class="text-white">"value = tween.value();"</span><span class="text-white/40">" // 0→100\n\n"</span>
    <span class="text-white/40">"// ...or set immediately:\n"</span>
    <span class="text-white">"tween.set("</span><span class="text-white">"50.0"</span><span class="text-white">");"</span>
                    </pre>
                </div>
                </div>

                <div class="flex-1 flex flex-col justify-center gap-8">
                    <div class="flex items-center justify-center p-12 min-h-[300px] bg-bg border-4 border-ink shadow-[12px_12px_0px_rgba(10,10,10,1)]">
                    <div style="width: 100%; text-align: center;">
                        <div class="font-instrument italic text-[5rem] text-accent-dark leading-none">
                            {move || format!("{:.1}", value.get())}
                        </div>
                        <div style="margin-top: 1.5rem; height: 6px; background: rgba(10,10,10,0.1); width: 100%;">
                            <div style=move || format!("height: 100%; background: var(--color-ink); transition: none; width: {}%;", value.get()) />
                        </div>
                        <button
                            class="bg-black text-accent font-mono uppercase px-8 py-3 border border-black inline-flex items-center gap-2 hover:bg-accent-dark hover:text-black transition-colors rounded-none mt-8"
                            on:click=play
                        >
                            {move || if running.get() { "Animating..." } else { "▶ Run Tween" }}
                        </button>
                    </div>
                    </div>
                </div>
            </div>
        }
}

#[component]
fn SpringExample() -> impl IntoView {
    let (pos, set_pos) = signal(0.0_f32);
    let (settled, set_settled) = signal(true);
    let (target_right, set_target_right) = signal(false);

    let current_id: Rc<std::cell::Cell<u32>> = Rc::new(std::cell::Cell::new(0));

    let spring = Rc::new(RefCell::new(
        Spring::new(SpringConfig::wobbly()).with_position(pos.get()),
    ));

    let play = {
        let current_id = current_id.clone();
        let spring = spring.clone();
        move |_| {
            let my_id = current_id.get() + 1;
            current_id.set(my_id);

            let target = if target_right.get() { 0.0 } else { 100.0 };
            set_target_right.set(!target_right.get());
            set_settled.set(false);

            {
                let mut s = spring.borrow_mut();
                s.set_target(target);
            }

            let sc = spring.clone();
            let current_id_clone = current_id.clone();
            animation::start_cancellable_raf_loop(my_id, current_id_clone, move |dt| {
                let mut s = sc.borrow_mut();
                if !s.is_settled() {
                    s.update(dt);
                    set_pos.set(s.position());
                } else {
                    set_settled.set(true);
                }
            });
        }
    };

    view! {
            <div class="flex flex-col lg:flex-row gap-12 mt-16 mb-24 max-w-[1300px] mx-auto reveal">
                <div class="flex-1 lg:sticky lg:top-[120px] h-fit">
                    <div class="bg-code-bg border-l-4 border-accent text-white font-mono rounded-none">
                    <div class="flex items-center gap-2 px-5 py-3 border-b border-white/10 bg-white/5">
                        <div class="w-2.5 h-2.5 rounded-full bg-[#ff5f57]" />
                        <div class="w-2.5 h-2.5 rounded-full bg-[#febc2e]" />
                        <div class="w-2.5 h-2.5 rounded-full bg-[#28c840]" />
                        <span class="ml-3 text-xs text-white/40 font-mono tracking-widest">"spring.rs"</span>
                    </div>
                    <pre class="p-6 text-sm leading-relaxed overflow-x-auto text-white">
    <span class="text-accent">"use "</span><span class="text-accent">"spanda"</span><span class="text-white">"::spring::{"</span><span class="text-accent">"Spring"</span><span class="text-white">", "</span><span class="text-accent">"SpringConfig"</span><span class="text-white">"};\n\n"</span>
    <span class="text-accent">"let mut "</span><span class="text-white">"spring = "</span><span class="text-accent">"Spring"</span><span class="text-white">"::new(\n"</span>
    <span class="text-white">"    "</span><span class="text-accent">"SpringConfig"</span><span class="text-white">"::wobbly()\n"</span>
    <span class="text-white">");\n"</span>
    <span class="text-white">"spring.set_target("</span><span class="text-white">"200.0"</span><span class="text-white">"); "</span><span class="text-white/40">"// 🏀\n\n"</span>
    <span class="text-white/40">"// Physics step each frame:\n"</span>
    <span class="text-white">"spring.update("</span><span class="text-white">"1.0 / 60.0"</span><span class="text-white">");\n"</span>
    <span class="text-accent">"let "</span><span class="text-white">"x = spring.position();"</span>
                    </pre>
                </div>
                </div>

                <div class="flex-1 flex flex-col justify-center gap-8">
                    <div class="flex items-center justify-center p-12 min-h-[300px] bg-bg border-[4px] border-ink shadow-[12px_12px_0px_rgba(10,10,10,1)]">
                    <div style="width: 100%; text-align: center;">
                        <div style="position: relative; height: 80px; background: rgba(10,10,10,0.1); border: 2px solid var(--color-border); overflow: hidden;">
                            <div
                                style=move || {
                                    let r = pos.get() / 100.0;
                                    format!(
                                        "position: absolute; left: calc(20px + {} * (100% - 80px)); top: 50%; \
                                         transform: translateY(-50%); width: 40px; height: 40px; background: var(--color-ink); border-radius: 50%; \
                                         flex-shrink: 0; transition: none;",
                                        r
                                    )
                                }
                            />
                        </div>
                        <button
                            class="bg-black text-accent font-mono uppercase px-8 py-3 border border-black inline-flex items-center gap-2 hover:bg-accent-dark hover:text-black transition-colors rounded-none mt-8"
                            on:click=play
                        >
                            {move || if !settled.get() { "Bouncing..." } else { "🏀 Spring Bounce" }}
                        </button>
                    </div>
                    </div>
                </div>
            </div>
        }
}

#[component]
fn TimelineExample() -> impl IntoView {
    let (p, set_p) = signal(0.0_f32);
    let (running, set_running) = signal(false);

    let play = move |_| {
        if running.get() {
            return;
        }
        set_running.set(true);
        set_p.set(0.0);

        let tween = Rc::new(RefCell::new(
            Tween::new(0.0_f32, 1.0)
                .duration(2.0)
                .easing(Easing::EaseInOutCubic)
                .build(),
        ));

        let tc = tween.clone();
        animation::start_raf_loop(move |dt| {
            let mut t = tc.borrow_mut();
            if *t.state() != TweenState::Completed {
                t.update(dt);
                set_p.set(t.value());
            } else {
                set_running.set(false);
            }
        });
    };

    view! {
            <div class="flex flex-col lg:flex-row gap-12 mt-16 mb-24 max-w-[1300px] mx-auto reveal">
                <div class="flex-1 lg:sticky lg:top-[120px] h-fit">
                    <div class="bg-code-bg border-l-4 border-accent text-white font-mono rounded-none">
                    <div class="flex items-center gap-2 px-5 py-3 border-b border-white/10 bg-white/5">
                        <div class="w-2.5 h-2.5 rounded-full bg-[#ff5f57]" />
                        <div class="w-2.5 h-2.5 rounded-full bg-[#febc2e]" />
                        <div class="w-2.5 h-2.5 rounded-full bg-[#28c840]" />
                        <span class="ml-3 text-xs text-white/40 font-mono tracking-widest">"timeline.rs"</span>
                    </div>
                    <pre class="p-6 text-sm leading-relaxed overflow-x-auto text-white">
    <span class="text-accent">"use "</span><span class="text-accent">"spanda"</span><span class="text-white">"::timeline::"</span><span class="text-accent">"Sequence"</span><span class="text-white">";\n\n"</span>
    <span class="text-accent">"let mut "</span><span class="text-white">"tl = "</span><span class="text-accent">"Sequence"</span><span class="text-white">"::new()\n"</span>
    <span class="text-white">"    .then(fade_in, "</span><span class="text-white">"0.5"</span><span class="text-white">")\n"</span>
    <span class="text-white">"    .then(slide_up, "</span><span class="text-white">"0.8"</span><span class="text-white">")\n"</span>
    <span class="text-white">"    .add_pause()\n"</span>
    <span class="text-white">"    .call(|| "</span><span class="text-accent">"println!"</span><span class="text-white">"("</span><span class="text-white">"\"Midway!\""</span><span class="text-white">"))\n"</span>
    <span class="text-white">"    .then(scale_in, "</span><span class="text-white">"0.3"</span><span class="text-white">")\n"</span>
    <span class="text-white">"    .build();\n\n"</span>
    <span class="text-white">"tl.play();"</span>
                    </pre>
                </div>
                </div>

                <div class="flex-1 flex flex-col justify-center gap-8">
                    <div class="flex items-center justify-center p-12 min-h-[300px] bg-bg border-4 border-ink shadow-[12px_12px_0px_rgba(10,10,10,1)]">
                    <div style="width: 100%; text-align: center;">
                        <div style="display: flex; gap: 1rem; justify-content: center; align-items: flex-end; height: 120px;">
                            <div style:opacity=move || {
                                let t = (p.get() * 3.0).min(1.0);
                                format!("{}", t)
                            } style:transform=move || {
                                let t = (p.get() * 3.0).min(1.0);
                                format!("translateY({}px)", (1.0 - t) * 30.0)
                            } style="width: 60px; height: 60px; border-radius: 0; background: var(--color-ink); border: 4px solid var(--color-accent); transition: none;" />

                            <div style:opacity=move || {
                                let t = ((p.get() - 0.2).max(0.0) * 3.3).min(1.0);
                                format!("{}", t)
                            } style:transform=move || {
                                let t = ((p.get() - 0.2).max(0.0) * 3.3).min(1.0);
                                format!("translateY({}px) scale({})", (1.0 - t) * 40.0, 0.5 + t * 0.5)
                            } style="width: 60px; height: 60px; border-radius: 50%; background: var(--color-accent); border: 4px solid var(--color-ink); transition: none;" />

                            <div style:opacity=move || {
                                let t = ((p.get() - 0.5).max(0.0) * 2.0).min(1.0);
                                format!("{}", t)
                            } style:transform=move || {
                                let t = ((p.get() - 0.5).max(0.0) * 2.0).min(1.0);
                                format!("scale({})", t)
                            } style="width: 60px; height: 60px; border-radius: 0; border: 4px solid var(--color-ink); transition: none;" />
                        </div>

                        <button
                            class="bg-black text-accent font-mono uppercase px-8 py-3 border border-black inline-flex items-center gap-2 hover:bg-accent-dark hover:text-black transition-colors rounded-none mt-8"
                            on:click=play
                        >
                            {move || if running.get() { "Sequencing..." } else { "▶ Run Sequence" }}
                        </button>
                    </div>
                    </div>
                </div>
            </div>
        }
}

#[component]
fn GpuComputeExample() -> impl IntoView {
    let (time, set_time) = signal(0.0f32);

    crate::animation::start_raf_loop(move |dt| {
        set_time.update(|t| *t += dt);
    });

    view! {
            <div class="flex flex-col lg:flex-row gap-12 mt-16 mb-24 max-w-[1300px] mx-auto reveal">
                <div class="flex-1 lg:sticky lg:top-[120px] h-fit">
                    <div class="bg-code-bg border-l-4 border-accent text-white font-mono rounded-none">
                    <div class="flex items-center gap-2 px-5 py-3 border-b border-white/10 bg-white/5">
                        <div class="w-2.5 h-2.5 rounded-full bg-[#ff5f57]" />
                        <div class="w-2.5 h-2.5 rounded-full bg-[#febc2e]" />
                        <div class="w-2.5 h-2.5 rounded-full bg-[#28c840]" />
                        <span class="ml-3 text-xs text-white/40 font-mono tracking-widest">"gpu_batch.rs"</span>
                    </div>
                    <pre class="p-6 text-sm leading-relaxed overflow-x-auto text-white">
    <span class="text-accent">"use "</span><span class="text-accent">"spanda"</span><span class="text-white">"::gpu::{"</span><span class="text-accent">"GpuContext"</span><span class="text-white">", "</span><span class="text-accent">"GpuAnimationBatch"</span><span class="text-white">"};\n\n"</span>
    <span class="text-white/40">"// Auto-detect adapter or fallback to CPU seamlessly\n"</span>
    <span class="text-accent">"let "</span><span class="text-white">"ctx = "</span><span class="text-accent">"GpuContext"</span><span class="text-white">"::new_auto()."</span><span class="text-accent">"await"</span><span class="text-white">";\n"</span>
    <span class="text-accent">"let mut "</span><span class="text-white">"batch = "</span><span class="text-accent">"GpuAnimationBatch"</span><span class="text-white">"::new(&ctx, "</span><span class="text-white">"10000"</span><span class="text-white">");\n\n"</span>
    <span class="text-white/40">"// Add thousands of tweens\n"</span>
    <span class="text-accent">"for"</span><span class="text-white">" _ "</span><span class="text-accent">"in"</span><span class="text-white">" "</span><span class="text-white">"0"</span><span class="text-white">".."</span><span class="text-white">"10000"</span><span class="text-white">" {\n"</span>
    <span class="text-white">"    batch.add("</span><span class="text-accent">"Tween"</span><span class="text-white">"::new("</span><span class="text-white">"0.0"</span><span class="text-white">", "</span><span class="text-white">"100.0"</span><span class="text-white">").duration("</span><span class="text-white">"2.0"</span><span class="text-white">").easing("</span><span class="text-accent">"Easing"</span><span class="text-white">"::Linear));\n"</span>
    <span class="text-white">"}\n\n"</span>
    <span class="text-white/40">"// Evaluate all on GPU in a single compute pass\n"</span>
    <span class="text-white">"batch.update_all(&ctx, dt)."</span><span class="text-accent">"await"</span><span class="text-white">";\n"</span>
    <span class="text-accent">"let "</span><span class="text-white">"results = batch.read_values(&ctx)."</span><span class="text-accent">"await"</span><span class="text-white">";"</span>
                    </pre>
                </div>
                </div>

                <div class="flex-1 flex flex-col justify-center gap-8">
                    <div class="flex items-center justify-center p-12 min-h-[300px] bg-bg border-4 border-ink shadow-[12px_12px_0px_rgba(10,10,10,1)]">
                    <div style="width: 100%; text-align: center;">
                        <div style="display: flex; gap: 4px; flex-wrap: wrap; justify-content: center; opacity: 1.0; max-height: 100px; overflow: hidden; align-content: flex-start;">
                            <For
                                each=move || 0..80
                                key=|i| *i
                                children=move |i| {
                                    let bg = if i % 3 == 0 { "var(--color-ink)" } else if i % 3 == 1 { "var(--color-accent)" } else { "var(--color-accent-dark)" };
                                    let size = 4 + (i % 4) * 2;
                                    let phase_x = (i as f32 * 0.5).sin();
                                    let phase_y = (i as f32 * 0.7).cos();
                                    let phase_speed = 1.0 + (i as f32 % 5.0) * 0.2;
                                    view! {
                                        <div style=move || {
                                            let t = time.get() * phase_speed * 2.0;
                                            let x = (t + phase_x * 10.0).sin() * 8.0;
                                            let y = (t + phase_y * 10.0).cos() * 8.0;
                                            format!("width: {}px; height: {}px; border-radius: 0; background: {}; opacity: {}; transform: translate({x}px, {y}px);", size, size, bg, 0.4 + (i as f32 % 10.0) / 20.0)
                                        }></div>
                                    }
                                }
                            />
                        </div>
                        <div class="font-mono mt-8 font-bold text-accent-dark tracking-widest text-xs">
                            "// 10,000 PARTICLES EVALUATED ON GPU"
                        </div>
                    </div>
                    </div>
                </div>
            </div>
        }
}
