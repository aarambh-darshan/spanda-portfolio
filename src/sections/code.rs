use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use spanda::{Tween, Easing, Spring, SpringConfig};
use spanda::traits::Update as _;
use spanda::tween::TweenState;
use crate::animation;

#[component]
pub fn CodeExamples() -> impl IntoView {
    view! {
        <section class="section" id="code">
            <div class="reveal">
                <span class="section-label">"Quick Start"</span>
                <h2 class="section-title">"Simple, expressive API."</h2>
                <p class="section-desc">
                    "Add spanda to your Cargo.toml and start animating in seconds. Builder pattern, fluent API, zero boilerplate."
                </p>
            </div>

            // ── Example 1: Basic Tween ──
            <TweenExample />

            // ── Example 2: Spring ──
            <SpringExample />

            // ── Example 3: Timeline ──
            <TimelineExample />
        </section>
    }
}

#[component]
fn TweenExample() -> impl IntoView {
    let (value, set_value) = signal(0.0_f32);
    let (running, set_running) = signal(false);

    let play = move |_| {
        if running.get() { return; }
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
        <div class="code-example reveal">
            <div class="code-block">
                <div class="code-header">
                    <div class="code-dot red" />
                    <div class="code-dot yellow" />
                    <div class="code-dot green" />
                    <span class="code-filename">"tween.rs"</span>
                </div>
                <pre class="code-body">
<span class="kw">"use "</span><span class="ty">"spanda"</span><span class="kw">"::{"</span><span class="ty">"Tween"</span><span class="kw">", "</span><span class="ty">"Easing"</span><span class="kw">"};\n"</span>
<span class="kw">"use "</span><span class="ty">"spanda"</span><span class="kw">"::traits::"</span><span class="ty">"Update"</span><span class="kw">";\n\n"</span>
<span class="kw">"let mut "</span><span>"tween = "</span><span class="ty">"Tween"</span><span>"::new("</span><span class="num">"0.0"</span><span>", "</span><span class="num">"100.0"</span><span>")\n"</span>
<span>"    .duration("</span><span class="num">"1.5"</span><span>")\n"</span>
<span>"    .easing("</span><span class="ty">"Easing"</span><span>"::EaseOutCubic)\n"</span>
<span>"    .build();\n\n"</span>
<span class="cm">"// Each frame:\n"</span>
<span>"tween.update(dt);\n"</span>
<span class="kw">"let "</span><span>"value = tween.value();"</span><span class="cm">" // 0→100"</span>
                </pre>
            </div>

            <div class="code-preview">
                <div style="width: 100%; text-align: center;">
                    <div style="font-family: 'Space Grotesk', sans-serif; font-size: 3rem; font-weight: 700; color: #ff6b35;">
                        {move || format!("{:.1}", value.get())}
                    </div>
                    <div class="tween-bar-track" style="margin-top: 1rem;">
                        <div class="tween-bar" style:width=move || format!("{}%", value.get()) />
                    </div>
                    <button
                        class="btn-primary"
                        style="margin-top: 1.5rem; padding: 0.5rem 1.5rem; font-size: 0.85rem;"
                        on:click=play
                    >
                        {move || if running.get() { "Animating..." } else { "▶ Run Tween" }}
                    </button>
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

    // Shared animation ID — prevents old loops from fighting new ones
    let current_id: Rc<std::cell::Cell<u32>> = Rc::new(std::cell::Cell::new(0));

    // Persistent spring instance so velocity is preserved across clicks
    let spring = Rc::new(RefCell::new(
        Spring::new(SpringConfig::wobbly()).with_position(pos.get())
    ));

    let play = {
        let current_id = current_id.clone();
        let spring = spring.clone();
        move |_| {
            // Increment ID to start a fresh animation loop execution
            let my_id = current_id.get() + 1;
            current_id.set(my_id);

            let target = if target_right.get() { 0.0 } else { 100.0 };
            set_target_right.set(!target_right.get());
            set_settled.set(false);

            // Just update the target, DO NOT recreate the spring (keeps velocity)
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
        <div class="code-example reveal">
            <div class="code-block">
                <div class="code-header">
                    <div class="code-dot red" />
                    <div class="code-dot yellow" />
                    <div class="code-dot green" />
                    <span class="code-filename">"spring.rs"</span>
                </div>
                <pre class="code-body">
<span class="kw">"use "</span><span class="ty">"spanda"</span><span class="kw">"::spring::{"</span><span class="ty">"Spring"</span><span class="kw">", "</span><span class="ty">"SpringConfig"</span><span class="kw">"};\n\n"</span>
<span class="kw">"let mut "</span><span>"spring = "</span><span class="ty">"Spring"</span><span>"::new(\n"</span>
<span>"    "</span><span class="ty">"SpringConfig"</span><span>"::wobbly()\n"</span>
<span>");\n"</span>
<span>"spring.set_target("</span><span class="num">"200.0"</span><span>"); "</span><span class="cm">"// 🏀\n\n"</span>
<span class="cm">"// Physics step each frame:\n"</span>
<span>"spring.update("</span><span class="num">"1.0 / 60.0"</span><span>");\n"</span>
<span class="kw">"let "</span><span>"x = spring.position();"</span>
                </pre>
            </div>

            <div class="code-preview">
                <div style="width: 100%; text-align: center;">
                    <div style="position: relative; height: 80px; background: rgba(0,0,0,0.3); border-radius: 12px; overflow: hidden;">
                        <div
                            class="demo-ball"
                            style=move || {
                                let r = pos.get() / 100.0;
                                format!(
                                    "position: absolute; left: calc(20px + {} * (100% - 80px)); top: 50%; \
                                     transform: translateY(-50%); width: 40px; height: 40px; background: #FF6B00; border-radius: 50%; \
                                     flex-shrink: 0; transition: none;",
                                    r
                                )
                            }
                        />
                    </div>
                    <button
                        class="btn-primary"
                        style="margin-top: 1.5rem; padding: 0.5rem 1.5rem; font-size: 0.85rem;"
                        on:click=play
                    >
                        {move || if !settled.get() { "Bouncing..." } else { "🏀 Spring Bounce" }}
                    </button>
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
        if running.get() { return; }
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
        <div class="code-example reveal">
            <div class="code-block">
                <div class="code-header">
                    <div class="code-dot red" />
                    <div class="code-dot yellow" />
                    <div class="code-dot green" />
                    <span class="code-filename">"timeline.rs"</span>
                </div>
                <pre class="code-body">
<span class="kw">"use "</span><span class="ty">"spanda"</span><span class="kw">"::timeline::"</span><span class="ty">"Sequence"</span><span class="kw">";\n\n"</span>
<span class="kw">"let mut "</span><span>"tl = "</span><span class="ty">"Sequence"</span><span>"::new()\n"</span>
<span>"    .then(fade_in, "</span><span class="num">"0.5"</span><span>")\n"</span>
<span>"    .then(slide_up, "</span><span class="num">"0.8"</span><span>")\n"</span>
<span>"    .gap("</span><span class="num">"0.1"</span><span>")\n"</span>
<span>"    .then(scale_in, "</span><span class="num">"0.3"</span><span>")\n"</span>
<span>"    .build();\n\n"</span>
<span>"tl.play();"</span>
                </pre>
            </div>

            <div class="code-preview">
                <div style="width: 100%; text-align: center;">
                    <div style="display: flex; gap: 1rem; justify-content: center; align-items: flex-end; height: 120px;">
                        <div style:opacity=move || {
                            let t = (p.get() * 3.0).min(1.0);
                            format!("{}", t)
                        } style:transform=move || {
                            let t = (p.get() * 3.0).min(1.0);
                            format!("translateY({}px)", (1.0 - t) * 30.0)
                        } style="width: 60px; height: 60px; border-radius: 12px; background: linear-gradient(135deg, #ff6b35, #e85d26); transition: none;" />

                        <div style:opacity=move || {
                            let t = ((p.get() - 0.2).max(0.0) * 3.3).min(1.0);
                            format!("{}", t)
                        } style:transform=move || {
                            let t = ((p.get() - 0.2).max(0.0) * 3.3).min(1.0);
                            format!("translateY({}px) scale({})", (1.0 - t) * 40.0, 0.5 + t * 0.5)
                        } style="width: 60px; height: 60px; border-radius: 50%; background: linear-gradient(135deg, #f7c948, #e8b530); transition: none;" />

                        <div style:opacity=move || {
                            let t = ((p.get() - 0.5).max(0.0) * 2.0).min(1.0);
                            format!("{}", t)
                        } style:transform=move || {
                            let t = ((p.get() - 0.5).max(0.0) * 2.0).min(1.0);
                            format!("scale({})", t)
                        } style="width: 60px; height: 60px; border-radius: 8px; background: linear-gradient(135deg, #56b6c2, #3a8a94); transition: none;" />
                    </div>

                    <button
                        class="btn-primary"
                        style="margin-top: 1.5rem; padding: 0.5rem 1.5rem; font-size: 0.85rem;"
                        on:click=play
                    >
                        {move || if running.get() { "Sequencing..." } else { "▶ Run Sequence" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
