use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use spanda::{Tween, Easing, Spring, SpringConfig};
use spanda::tween::TweenState;
use spanda::traits::Update as _;
use crate::animation;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DemoTab {
    Easing,
    Spring,
    SvgDraw,
    Morph,
    Timeline,
    SplitText,
}

#[component]
pub fn Demos() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(DemoTab::Easing);

    view! {
        <section class="section" id="demos">
            <div class="reveal">
                <span class="section-label">"Interactive"</span>
                <h2 class="section-title">"Spanda in motion."</h2>
                <p class="section-desc">
                    "See Spanda's animation primitives running live — powered by real Rust/WASM code."
                </p>
            </div>

            <div class="demo-tabs" style="margin-top: 3rem;">
                <DemoTabBtn tab=DemoTab::Easing active=active_tab set_active=set_active_tab label="Easing Curves" />
                <DemoTabBtn tab=DemoTab::Spring active=active_tab set_active=set_active_tab label="Spring Physics" />
                <DemoTabBtn tab=DemoTab::SvgDraw active=active_tab set_active=set_active_tab label="SVG Draw" />
                <DemoTabBtn tab=DemoTab::Morph active=active_tab set_active=set_active_tab label="Shape Morph" />
                <DemoTabBtn tab=DemoTab::Timeline active=active_tab set_active=set_active_tab label="Timeline" />
                <DemoTabBtn tab=DemoTab::SplitText active=active_tab set_active=set_active_tab label="Split Text" />
            </div>

            <div class="demo-panel">
                {move || match active_tab.get() {
                    DemoTab::Easing => view! { <EasingDemo /> }.into_any(),
                    DemoTab::Spring => view! { <SpringDemo /> }.into_any(),
                    DemoTab::SvgDraw => view! { <SvgDrawDemo /> }.into_any(),
                    DemoTab::Morph => view! { <MorphDemo /> }.into_any(),
                    DemoTab::Timeline => view! { <TimelineDemo /> }.into_any(),
                    DemoTab::SplitText => view! { <SplitTextDemo /> }.into_any(),
                }}
            </div>
        </section>
    }
}

#[component]
fn DemoTabBtn(
    tab: DemoTab,
    active: ReadSignal<DemoTab>,
    set_active: WriteSignal<DemoTab>,
    label: &'static str,
) -> impl IntoView {
    view! {
        <button
            class=move || if active.get() == tab { "demo-tab active" } else { "demo-tab" }
            on:click=move |_| set_active.set(tab)
        >
            {label}
        </button>
    }
}

// ── Easing Demo ─────────────────────────────────────────────────────────

const EASING_NAMES: &[(&str, fn() -> Easing)] = &[
    ("Linear",         || Easing::Linear),
    ("EaseInQuad",     || Easing::EaseInQuad),
    ("EaseOutQuad",    || Easing::EaseOutQuad),
    ("EaseInOutCubic", || Easing::EaseInOutCubic),
    ("EaseOutCubic",   || Easing::EaseOutCubic),
    ("EaseInQuart",    || Easing::EaseInQuart),
    ("EaseOutQuart",   || Easing::EaseOutQuart),
    ("EaseInOutExpo",  || Easing::EaseInOutExpo),
    ("EaseOutExpo",    || Easing::EaseOutExpo),
    ("EaseInBack",     || Easing::EaseInBack),
    ("EaseOutBack",    || Easing::EaseOutBack),
    ("EaseInOutBack",  || Easing::EaseInOutBack),
    ("EaseOutElastic", || Easing::EaseOutElastic),
    ("EaseOutBounce",  || Easing::EaseOutBounce),
    ("EaseInOutBounce",|| Easing::EaseInOutBounce),
];

#[component]
fn EasingDemo() -> impl IntoView {
    let (selected, set_selected) = signal(0_usize);
    let (progress, set_progress) = signal(0.0_f32);
    let (running, set_running) = signal(false);

    // SVG path for the curve — uses viewBox 0 0 600 500 for taller canvas
    // Y range: map value [-0.3, 1.3] → [490, 10] to handle overshoot
    let curve_path = Memo::new(move |_| {
        let idx = selected.get();
        let easing_fn = (EASING_NAMES[idx].1)();
        let mut d = String::new();
        for i in 0..=200 {
            let t = i as f32 / 200.0;
            let v = easing_fn.apply(t);
            let x = 20.0 + t * 560.0;
            // Map value range [-0.3, 1.3] → [490, 10]
            let y = 490.0 - (v + 0.3) / 1.6 * 480.0;
            if i == 0 {
                d.push_str(&format!("M {:.1} {:.1}", x, y));
            } else {
                d.push_str(&format!(" L {:.1} {:.1}", x, y));
            }
        }
        d
    });

    // For the dot: we tween a separate raw progress signal
    let (raw_t, set_raw_t) = signal(0.0_f32);

    let play_with_tracking = move |_ev: web_sys::MouseEvent| {
        if running.get() { return; }
        set_running.set(true);
        set_progress.set(0.0);
        set_raw_t.set(0.0);

        let idx = selected.get();
        let easing = (EASING_NAMES[idx].1)();

        // Linear tween for tracking raw progress
        let raw_tween = Rc::new(RefCell::new(
            Tween::new(0.0_f32, 1.0)
                .duration(1.8)
                .easing(Easing::Linear)
                .build(),
        ));
        // Eased tween for the actual animation value
        let eased_tween = Rc::new(RefCell::new(
            Tween::new(0.0_f32, 1.0)
                .duration(1.8)
                .easing(easing)
                .build(),
        ));

        let rt = raw_tween.clone();
        let et = eased_tween.clone();
        animation::start_raf_loop(move |dt| {
            let mut r = rt.borrow_mut();
            let mut e = et.borrow_mut();
            if *r.state() != TweenState::Completed {
                r.update(dt);
                e.update(dt);
                set_raw_t.set(r.value());
                set_progress.set(e.value());
            } else {
                set_raw_t.set(1.0);
                set_progress.set(e.value());
                set_running.set(false);
            }
        });
    };

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                <h3 style="font-size: 1.1rem; font-weight: 600;">"Easing Curve Visualizer"</h3>
                <button class="btn-primary" style="padding: 0.5rem 1.5rem; font-size: 0.85rem;" on:click=play_with_tracking>
                    {move || if running.get() { "Playing..." } else { "▶ Play" }}
                </button>
            </div>

            // Taller canvas
            <div class="easing-canvas" style="height: 450px; position: relative;">
                <svg viewBox="0 0 600 500" style="width: 100%; height: 100%;" preserveAspectRatio="none">
                    // Grid lines — y=0 and y=1 reference
                    <line x1="20" y1={format!("{}", 490.0 - 0.3 / 1.6 * 480.0)} x2="580"
                          y2={format!("{}", 490.0 - 0.3 / 1.6 * 480.0)}
                          stroke="rgba(255,255,255,0.08)" stroke-width="1" stroke-dasharray="4,4" />
                    <line x1="20" y1={format!("{}", 490.0 - 1.3 / 1.6 * 480.0)} x2="580"
                          y2={format!("{}", 490.0 - 1.3 / 1.6 * 480.0)}
                          stroke="rgba(255,255,255,0.08)" stroke-width="1" stroke-dasharray="4,4" />
                    // Labels
                    <text x="5" y={format!("{}", 490.0 - 0.3 / 1.6 * 480.0 + 4.0)}
                          fill="rgba(255,255,255,0.2)" font-size="10" font-family="JetBrains Mono">"0"</text>
                    <text x="5" y={format!("{}", 490.0 - 1.3 / 1.6 * 480.0 + 4.0)}
                          fill="rgba(255,255,255,0.2)" font-size="10" font-family="JetBrains Mono">"1"</text>
                    // Curve
                    <path d=curve_path fill="none" stroke="#ff6b35" stroke-width="2.5" opacity="0.9" />
                </svg>
                // Animated dot — follows along the curve correctly
                <div
                    class="easing-dot"
                    style=move || {
                        let t = raw_t.get();
                        let v = progress.get();
                        // Map t → x position (percentage)
                        let x_pct = t * 100.0;
                        // Map v → y position: same mapping as SVG
                        let y_pct = ((v + 0.3) / 1.6) * 100.0;
                        format!(
                            "position: absolute; left: calc({}% + 7px); bottom: calc({}% - 7px); \
                             width: 14px; height: 14px; border-radius: 50%; background: #ff6b35; \
                             box-shadow: 0 0 20px rgba(255,107,53,0.5); pointer-events: none; transition: none;",
                            x_pct * 0.933 + 3.33,  // adjust for 20px padding
                            y_pct * 0.96 + 2.0
                        )
                    }
                />
            </div>

            // Easing value display
            <div style="display: flex; justify-content: space-between; margin-top: 0.5rem; font-size: 0.75rem; font-family: 'JetBrains Mono', monospace; color: rgba(255,255,255,0.3);">
                <span>"t: " {move || format!("{:.2}", raw_t.get())}</span>
                <span>"value: " {move || format!("{:.3}", progress.get())}</span>
            </div>

            <div class="easing-selector" style="margin-top: 1rem;">
                {EASING_NAMES.iter().enumerate().map(|(i, (name, _))| {
                    let name = *name;
                    view! {
                        <button
                            class=move || if selected.get() == i { "easing-option active" } else { "easing-option" }
                            on:click=move |_| set_selected.set(i)
                        >
                            {name}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

// ── Spring Demo ─────────────────────────────────────────────────────────

#[component]
fn SpringDemo() -> impl IntoView {
    let (stiffness, set_stiffness) = signal(180.0_f32);
    let (damping, set_damping) = signal(12.0_f32);
    let (mass, set_mass) = signal(1.0_f32);
    let (position, set_position) = signal(0.0_f32);
    let (is_right, set_is_right) = signal(false);

    // Shared animation ID — cancels previous animation when a new one starts
    let current_id: Rc<std::cell::Cell<u32>> = Rc::new(std::cell::Cell::new(0));

    // Persistent spring instance so velocity is preserved across clicks
    let spring = Rc::new(RefCell::new(
        Spring::new(SpringConfig {
            stiffness: stiffness.get_untracked(),
            damping: damping.get_untracked(),
            mass: mass.get_untracked(),
            epsilon: 0.01,
        }).with_position(position.get_untracked())
    ));

    // Update config whenever sliders change
    {
        let spring = spring.clone();
        Effect::new(move |_| {
            let config = SpringConfig {
                stiffness: stiffness.get(),
                damping: damping.get(),
                mass: mass.get(),
                epsilon: 0.01,
            };
            spring.borrow_mut().config = config;
        });
    }

    let animate = {
        let current_id = current_id.clone();
        let spring = spring.clone();
        move |_| {
            // Increment ID to cancel previous loops
            let my_id = current_id.get() + 1;
            current_id.set(my_id);

            let target = if is_right.get() { 0.0 } else { 100.0 };
            set_is_right.set(!is_right.get());

            // Just update target, DO NOT recreate spring (keeps velocity)
            {
                let mut s = spring.borrow_mut();
                s.set_target(target);
            }

            let spring_clone = spring.clone();
            let current_id_clone = current_id.clone();
            animation::start_cancellable_raf_loop(my_id, current_id_clone, move |dt| {
                let mut s = spring_clone.borrow_mut();
                if !s.is_settled() {
                    s.update(dt);
                    set_position.set(s.position());
                }
            });
        }
    };

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                <h3 style="font-size: 1.1rem; font-weight: 600;">"Spring Physics Playground"</h3>
                <button class="btn-primary" style="padding: 0.5rem 1.5rem; font-size: 0.85rem;" on:click=animate>
                    "🏀 Bounce"
                </button>
            </div>

            <div class="spring-track">
                <div
                    class="spring-ball"
                    style=move || {
                        let r = position.get() / 100.0;
                        format!(
                            "position: absolute; left: calc({} * (100% - 50px)); top: 50%; transform: translateY(-50%); \
                             width: 50px; height: 50px; flex-shrink: 0; transition: none;",
                            r
                        )
                    }
                />
            </div>

            <div class="slider-group">
                <div>
                    <div class="slider-label">
                        <span>"Stiffness"</span>
                        <span>{move || format!("{:.0}", stiffness.get())}</span>
                    </div>
                    <input type="range" min="10" max="500" step="1"
                        prop:value=move || stiffness.get().to_string()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<f32>() {
                                set_stiffness.set(v);
                            }
                        }
                    />
                </div>
                <div>
                    <div class="slider-label">
                        <span>"Damping"</span>
                        <span>{move || format!("{:.0}", damping.get())}</span>
                    </div>
                    <input type="range" min="1" max="40" step="0.5"
                        prop:value=move || damping.get().to_string()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<f32>() {
                                set_damping.set(v);
                            }
                        }
                    />
                </div>
                <div>
                    <div class="slider-label">
                        <span>"Mass"</span>
                        <span>{move || format!("{:.1}", mass.get())}</span>
                    </div>
                    <input type="range" min="0.1" max="5.0" step="0.1"
                        prop:value=move || mass.get().to_string()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<f32>() {
                                set_mass.set(v);
                            }
                        }
                    />
                </div>
            </div>

            <div style="margin-top: 1.5rem; display: flex; gap: 0.5rem; flex-wrap: wrap;">
                <button class="easing-option" on:click=move |_| { set_stiffness.set(60.0); set_damping.set(14.0); set_mass.set(1.0); }>
                    "Gentle"
                </button>
                <button class="easing-option" on:click=move |_| { set_stiffness.set(180.0); set_damping.set(12.0); set_mass.set(1.0); }>
                    "Wobbly"
                </button>
                <button class="easing-option" on:click=move |_| { set_stiffness.set(210.0); set_damping.set(20.0); set_mass.set(1.0); }>
                    "Stiff"
                </button>
                <button class="easing-option" on:click=move |_| { set_stiffness.set(37.0); set_damping.set(14.0); set_mass.set(1.0); }>
                    "Slow"
                </button>
            </div>

            <p style="margin-top: 1rem; font-size: 0.75rem; color: rgba(255,255,255,0.3); font-family: 'JetBrains Mono', monospace;">
                "position: " {move || format!("{:.1}", position.get())}
                " · settled: " {move || format!("{}", position.get().round() == position.get())}
            </p>
        </div>
    }
}

// ── SVG Draw Demo — "spanda" text ───────────────────────────────────────

#[component]
fn SvgDrawDemo() -> impl IntoView {
    let (offset, set_offset) = signal(2000.0_f32);
    let (running, set_running) = signal(false);

    let play = move |_| {
        if running.get() { return; }
        set_running.set(true);
        set_offset.set(2000.0);

        let tween = Rc::new(RefCell::new(
            Tween::new(2000.0_f32, 0.0)
                .duration(3.0)
                .easing(Easing::EaseInOutCubic)
                .build(),
        ));

        let tween_clone = tween.clone();
        animation::start_raf_loop(move |dt| {
            let mut t = tween_clone.borrow_mut();
            if *t.state() != TweenState::Completed {
                t.update(dt);
                set_offset.set(t.value());
            } else {
                set_running.set(false);
            }
        });
    };

    let reset = move |_| {
        set_offset.set(2000.0);
        set_running.set(false);
    };

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                <h3 style="font-size: 1.1rem; font-weight: 600;">"SVG Path Drawing — \"spanda\""</h3>
                <div style="display: flex; gap: 0.5rem;">
                    <button class="btn-secondary" style="padding: 0.5rem 1rem; font-size: 0.85rem;" on:click=reset>
                        "↺ Reset"
                    </button>
                    <button class="btn-primary" style="padding: 0.5rem 1.5rem; font-size: 0.85rem;" on:click=play>
                        {move || if running.get() { "Drawing..." } else { "✏️ Draw" }}
                    </button>
                </div>
            </div>

            <div class="svg-draw-container">
                <svg viewBox="0 0 640 160" style="width: 100%; max-width: 640px;">
                    // "s" — curved
                    <path
                        d="M30,55 C30,30 70,25 70,50 C70,75 30,70 30,95 C30,120 70,125 70,100"
                        fill="none" stroke="#ff6b35" stroke-width="4" stroke-linecap="round"
                        stroke-dasharray="2000" stroke-dashoffset=move || offset.get()
                    />
                    // "p" — vertical + round
                    <path
                        d="M100,40 L100,140 M100,40 L100,45 C100,25 150,25 150,50 C150,75 100,75 100,55"
                        fill="none" stroke="#ff6b35" stroke-width="4" stroke-linecap="round"
                        stroke-dasharray="2000" stroke-dashoffset=move || offset.get()
                    />
                    // "a" — round + stem
                    <path
                        d="M230,95 C230,60 180,55 180,80 C180,105 230,110 230,80 L230,100"
                        fill="none" stroke="#ff6b35" stroke-width="4" stroke-linecap="round"
                        stroke-dasharray="2000" stroke-dashoffset=move || offset.get()
                    />
                    // "n" — vertical + arch
                    <path
                        d="M260,100 L260,55 C260,35 310,35 310,55 L310,100"
                        fill="none" stroke="#ff6b35" stroke-width="4" stroke-linecap="round"
                        stroke-dasharray="2000" stroke-dashoffset=move || offset.get()
                    />
                    // "d" — round + stem
                    <path
                        d="M390,95 C390,60 340,55 340,80 C340,105 390,110 390,80 L390,30 L390,100"
                        fill="none" stroke="#ff6b35" stroke-width="4" stroke-linecap="round"
                        stroke-dasharray="2000" stroke-dashoffset=move || offset.get()
                    />
                    // "a" — final a
                    <path
                        d="M470,95 C470,60 420,55 420,80 C420,105 470,110 470,80 L470,100"
                        fill="none" stroke="#ff6b35" stroke-width="4" stroke-linecap="round"
                        stroke-dasharray="2000" stroke-dashoffset=move || offset.get()
                    />
                </svg>
                <p style="font-size: 0.8rem; color: rgba(255,255,255,0.35); font-family: 'JetBrains Mono', monospace;">
                    "stroke-dashoffset: " {move || format!("{:.0}", offset.get())}
                    " · draw_on(path_len)"
                </p>
            </div>
        </div>
    }
}

// ── Morph Demo — multiple shapes ────────────────────────────────────────

fn make_polygon(sides: usize, cx: f32, cy: f32, r: f32, rotation: f32) -> Vec<[f32; 2]> {
    let mut pts = Vec::with_capacity(sides);
    for i in 0..sides {
        let angle = rotation + (i as f32 / sides as f32) * std::f32::consts::TAU;
        pts.push([cx + r * angle.cos(), cy + r * angle.sin()]);
    }
    pts
}

fn make_star(points: usize, cx: f32, cy: f32, r_outer: f32, r_inner: f32) -> Vec<[f32; 2]> {
    let mut pts = Vec::with_capacity(points * 2);
    for i in 0..(points * 2) {
        let angle = (i as f32 / (points * 2) as f32) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
        let r = if i % 2 == 0 { r_outer } else { r_inner };
        pts.push([cx + r * angle.cos(), cy + r * angle.sin()]);
    }
    pts
}

// Resample a polygon to have exactly `n` points
fn resample_to(pts: &[[f32; 2]], n: usize) -> Vec<[f32; 2]> {
    if pts.is_empty() { return vec![[0.0; 2]; n]; }
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / (n - 1).max(1) as f32;
        let idx = t * (pts.len() - 1) as f32;
        let lo = (idx.floor() as usize).min(pts.len() - 1);
        let hi = (lo + 1).min(pts.len() - 1);
        let frac = idx - lo as f32;
        result.push([
            pts[lo][0] + (pts[hi][0] - pts[lo][0]) * frac,
            pts[lo][1] + (pts[hi][1] - pts[lo][1]) * frac,
        ]);
    }
    result
}

const SHAPE_NAMES: &[&str] = &["Triangle", "Circle", "Star", "Hexagon", "Square", "Pentagon"];
const NUM_SAMPLE_POINTS: usize = 60;

#[component]
fn MorphDemo() -> impl IntoView {
    let cx = 150.0_f32;
    let cy = 140.0;
    let r = 100.0;

    // Pre-generate all shapes resampled to NUM_SAMPLE_POINTS
    let shapes: Vec<Vec<[f32; 2]>> = vec![
        resample_to(&make_polygon(3, cx, cy, r, -std::f32::consts::FRAC_PI_2), NUM_SAMPLE_POINTS),
        resample_to(&make_polygon(60, cx, cy, r, 0.0), NUM_SAMPLE_POINTS), // circle approx
        resample_to(&make_star(5, cx, cy, r, r * 0.4), NUM_SAMPLE_POINTS),
        resample_to(&make_polygon(6, cx, cy, r, -std::f32::consts::FRAC_PI_2), NUM_SAMPLE_POINTS),
        resample_to(&make_polygon(4, cx, cy, r, -std::f32::consts::FRAC_PI_4), NUM_SAMPLE_POINTS),
        resample_to(&make_polygon(5, cx, cy, r, -std::f32::consts::FRAC_PI_2), NUM_SAMPLE_POINTS),
    ];

    let (current_shape, set_current_shape) = signal(0_usize);
    let (target_shape, set_target_shape) = signal(0_usize);
    let (morph_t, set_morph_t) = signal(0.0_f32);
    let (running, set_running) = signal(false);

    let shapes_for_memo = shapes.clone();
    let points_str = Memo::new(move |_| {
        let t = morph_t.get();
        let from_idx = current_shape.get();
        let to_idx = target_shape.get();
        let from = &shapes_for_memo[from_idx];
        let to = &shapes_for_memo[to_idx];
        from.iter()
            .zip(to.iter())
            .map(|(a, b)| {
                let x = a[0] + (b[0] - a[0]) * t;
                let y = a[1] + (b[1] - a[1]) * t;
                format!("{:.1},{:.1}", x, y)
            })
            .collect::<Vec<_>>()
            .join(" ")
    });

    let morph_to = {
        move |idx: usize| {
            if running.get() || idx == target_shape.get() { return; }
            set_running.set(true);
            // Current morph result becomes the new "from"
            set_current_shape.set(target_shape.get());
            set_target_shape.set(idx);
            set_morph_t.set(0.0);

            let tween = Rc::new(RefCell::new(
                Tween::new(0.0_f32, 1.0)
                    .duration(1.0)
                    .easing(Easing::EaseInOutCubic)
                    .build(),
            ));

            let tween_clone = tween.clone();
            animation::start_raf_loop(move |dt| {
                let mut t = tween_clone.borrow_mut();
                if *t.state() != TweenState::Completed {
                    t.update(dt);
                    set_morph_t.set(t.value());
                } else {
                    set_morph_t.set(1.0);
                    set_running.set(false);
                }
            });
        }
    };

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                <h3 style="font-size: 1.1rem; font-weight: 600;">"Shape Morphing"</h3>
                <span style="font-size: 0.75rem; color: rgba(255,255,255,0.4); font-family: 'JetBrains Mono', monospace;">
                    {move || SHAPE_NAMES[target_shape.get()]}
                </span>
            </div>

            <div class="morph-container">
                <svg viewBox="0 0 300 280">
                    <polygon
                        points=points_str
                        fill="rgba(255, 107, 53, 0.12)"
                        stroke="#ff6b35"
                        stroke-width="2"
                        stroke-linejoin="round"
                    />
                </svg>

                <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; justify-content: center;">
                    {SHAPE_NAMES.iter().enumerate().map(|(i, name)| {
                        let name = *name;
                        let morph_fn = morph_to.clone();
                        view! {
                            <button
                                class=move || if target_shape.get() == i { "easing-option active" } else { "easing-option" }
                                on:click=move |_| morph_fn(i)
                            >
                                {name}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                <p style="font-size: 0.75rem; color: rgba(255,255,255,0.3); font-family: 'JetBrains Mono', monospace;">
                    "morph_t: " {move || format!("{:.2}", morph_t.get())}
                    " · " {move || format!("{} points", NUM_SAMPLE_POINTS)}
                </p>
            </div>
        </div>
    }
}

// ── Timeline Demo — visual animated blocks ──────────────────────────────

#[component]
fn TimelineDemo() -> impl IntoView {
    let (progress, set_progress) = signal(0.0_f32);
    let (running, set_running) = signal(false);

    let play = move |_| {
        if running.get() { return; }
        set_running.set(true);
        set_progress.set(0.0);

        let tween = Rc::new(RefCell::new(
            Tween::new(0.0_f32, 1.0)
                .duration(4.0)
                .easing(Easing::Linear)
                .build(),
        ));

        let tween_clone = tween.clone();
        animation::start_raf_loop(move |dt| {
            let mut t = tween_clone.borrow_mut();
            if *t.state() != TweenState::Completed {
                t.update(dt);
                set_progress.set(t.value());
            } else {
                set_progress.set(1.0);
                set_running.set(false);
            }
        });
    };

    // Timeline items: (label, start, end, color)
    let items: &[(&str, f32, f32, &str)] = &[
        ("fade_in",   0.0,  0.20, "#ff6b35"),
        ("slide_up",  0.10, 0.40, "#f7c948"),
        ("scale",     0.25, 0.55, "#56b6c2"),
        ("rotate",    0.40, 0.70, "#c678dd"),
        ("color",     0.55, 0.80, "#98c379"),
        ("bounce",    0.70, 1.00, "#e06c75"),
    ];

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                <h3 style="font-size: 1.1rem; font-weight: 600;">"Timeline Sequencing"</h3>
                <button class="btn-primary" style="padding: 0.5rem 1.5rem; font-size: 0.85rem;" on:click=play>
                    {move || if running.get() { "Playing..." } else { "▶ Play Timeline" }}
                </button>
            </div>

            // Playhead
            <div style="position: relative; margin-bottom: 1rem;">
                <div class="tween-bar-track" style="height: 6px;">
                    <div class="tween-bar" style:width=move || format!("{}%", progress.get() * 100.0) />
                </div>
                // Time markers
                <div style="display: flex; justify-content: space-between; margin-top: 0.25rem;">
                    <span style="font-size: 0.65rem; font-family: 'JetBrains Mono', monospace; color: rgba(255,255,255,0.25);">"0s"</span>
                    <span style="font-size: 0.65rem; font-family: 'JetBrains Mono', monospace; color: rgba(255,255,255,0.25);">"1s"</span>
                    <span style="font-size: 0.65rem; font-family: 'JetBrains Mono', monospace; color: rgba(255,255,255,0.25);">"2s"</span>
                    <span style="font-size: 0.65rem; font-family: 'JetBrains Mono', monospace; color: rgba(255,255,255,0.25);">"3s"</span>
                    <span style="font-size: 0.65rem; font-family: 'JetBrains Mono', monospace; color: rgba(255,255,255,0.25);">"4s"</span>
                </div>
            </div>

            // Track lanes
            <div class="timeline-track" style="gap: 0.5rem;">
                {items.iter().map(|(label, start, end, color)| {
                    let label = *label;
                    let start = *start;
                    let end = *end;
                    let color = *color;
                    view! {
                        <div class="timeline-item" style="gap: 0.75rem;">
                            <div class="timeline-item-label" style="width: 70px; font-size: 0.7rem;">{label}</div>
                            <div class="timeline-item-bar-track" style="height: 32px; border-radius: 6px;">
                                <div
                                    style=move || {
                                        let p = progress.get();
                                        let active = p >= start;
                                        let local_progress = if p >= end { 1.0 }
                                            else if p >= start { (p - start) / (end - start) }
                                            else { 0.0 };
                                        format!(
                                            "position: absolute; top: 3px; bottom: 3px; left: {}%; width: {}%; \
                                             border-radius: 4px; background: {}; opacity: {}; \
                                             box-shadow: 0 0 {}px {};",
                                            start * 100.0,
                                            (end - start) * 100.0,
                                            color,
                                            if active { 0.4 + local_progress * 0.6 } else { 0.15 },
                                            if active { (local_progress * 12.0) as i32 } else { 0 },
                                            color,
                                        )
                                    }
                                />
                                // Active fill within the bar
                                <div
                                    style=move || {
                                        let p = progress.get();
                                        let local_progress = if p >= end { 1.0 }
                                            else if p >= start { (p - start) / (end - start) }
                                            else { 0.0 };
                                        format!(
                                            "position: absolute; top: 3px; bottom: 3px; left: {}%; \
                                             width: {}%; border-radius: 4px; background: {}; opacity: 0.9;",
                                            start * 100.0,
                                            (end - start) * local_progress * 100.0,
                                            color,
                                        )
                                    }
                                />
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            // Animated result preview
            <div style="margin-top: 2rem; display: flex; align-items: center; justify-content: center; gap: 1rem; height: 80px;">
                {items.iter().map(|(label, start, end, color)| {
                    let start = *start;
                    let end = *end;
                    let color = *color;
                    let label = *label;
                    view! {
                        <div style=move || {
                            let p = progress.get();
                            let local_t = if p >= end { 1.0 }
                                else if p >= start { (p - start) / (end - start) }
                                else { 0.0 };

                            let (opacity, transform) = match label {
                                "fade_in" => (local_t, "none".to_string()),
                                "slide_up" => (local_t, format!("translateY({}px)", (1.0 - local_t) * 40.0)),
                                "scale" => (local_t, format!("scale({})", local_t)),
                                "rotate" => (local_t, format!("rotate({}deg)", local_t * 360.0)),
                                "color" => (1.0, "none".to_string()),
                                "bounce" => (1.0, format!("translateY({}px)", (1.0 - local_t) * 30.0 * ((local_t * 8.0).sin()))),
                                _ => (1.0, "none".to_string()),
                            };

                            let bg = if label == "color" {
                                let r = (255.0 * (1.0 - local_t) + 152.0 * local_t) as u8;
                                let g = (107.0 * (1.0 - local_t) + 195.0 * local_t) as u8;
                                let b = (53.0 * (1.0 - local_t) + 121.0 * local_t) as u8;
                                format!("rgb({},{},{})", r, g, b)
                            } else {
                                color.to_string()
                            };

                            format!(
                                "width: 45px; height: 45px; border-radius: 8px; background: {}; \
                                 opacity: {}; transform: {}; transition: none;",
                                bg, opacity, transform
                            )
                        } />
                    }
                }).collect::<Vec<_>>()}
            </div>

            <p style="margin-top: 0.5rem; text-align: center; font-size: 0.7rem; color: rgba(255,255,255,0.3); font-family: 'JetBrains Mono', monospace;">
                "timeline.elapsed(): " {move || format!("{:.1}s", progress.get() * 4.0)}
                " / 4.0s"
            </p>
        </div>
    }
}

// ── Split Text Demo ─────────────────────────────────────────────────────

#[component]
fn SplitTextDemo() -> impl IntoView {
    let (mode, set_mode) = signal(0_usize); // 0=chars, 1=words, 2=lines
    let demo_text = "Spanda brings animation to life in Rust.";
    let chars: Vec<char> = demo_text.chars().collect();
    let words: Vec<&str> = demo_text.split_whitespace().collect();

    // Opacity and Y signals for each character
    let char_ops: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..chars.len()).map(|_| signal(0.0_f32)).collect();
    let char_ys: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..chars.len()).map(|_| signal(30.0_f32)).collect();

    let word_ops: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..words.len()).map(|_| signal(0.0_f32)).collect();
    let word_ys: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..words.len()).map(|_| signal(25.0_f32)).collect();

    let (line_op, set_line_op) = signal(0.0_f32);
    let (line_y, set_line_y) = signal(40.0_f32);

    let (playing, set_playing) = signal(false);

    // Animate characters
    let char_w_ops: Vec<WriteSignal<f32>> = char_ops.iter().map(|(_, w)| *w).collect();
    let char_w_ys: Vec<WriteSignal<f32>> = char_ys.iter().map(|(_, w)| *w).collect();
    let word_w_ops: Vec<WriteSignal<f32>> = word_ops.iter().map(|(_, w)| *w).collect();
    let word_w_ys: Vec<WriteSignal<f32>> = word_ys.iter().map(|(_, w)| *w).collect();

    let play = {
        let char_w_ops = char_w_ops.clone();
        let char_w_ys = char_w_ys.clone();
        let word_w_ops = word_w_ops.clone();
        let word_w_ys = word_w_ys.clone();
        move |_: web_sys::MouseEvent| {
            if playing.get() { return; }
            set_playing.set(true);

            let m = mode.get();

            // Reset all
            for w in &char_w_ops { w.set(0.0); }
            for w in &char_w_ys { w.set(30.0); }
            for w in &word_w_ops { w.set(0.0); }
            for w in &word_w_ys { w.set(25.0); }
            set_line_op.set(0.0);
            set_line_y.set(40.0);

            match m {
                0 => {
                    // Character stagger
                    for (i, (set_op, set_y)) in char_w_ops.iter().zip(char_w_ys.iter()).enumerate() {
                        let set_op = *set_op;
                        let set_y = *set_y;
                        let total = char_w_ops.len();
                        let delay = (i as i32) * 30;
                        let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                            let ty = Rc::new(RefCell::new(
                                Tween::new(30.0_f32, 0.0).duration(0.5).easing(Easing::EaseOutExpo).build()
                            ));
                            let to = Rc::new(RefCell::new(
                                Tween::new(0.0_f32, 1.0).duration(0.4).easing(Easing::EaseOutCubic).build()
                            ));
                            let tty = ty.clone();
                            let tto = to.clone();
                            animation::start_raf_loop(move |dt| {
                                let mut y = tty.borrow_mut();
                                let mut o = tto.borrow_mut();
                                if *y.state() != TweenState::Completed {
                                    y.update(dt);
                                    o.update(dt);
                                    set_y.set(y.value());
                                    set_op.set(o.value());
                                }
                            });
                        });
                        let _ = web_sys::window().unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                cb.as_ref().unchecked_ref(), delay
                            );
                        if i == total - 1 {
                            let cb2 = wasm_bindgen::closure::Closure::once_into_js(move || {
                                set_playing.set(false);
                            });
                            let _ = web_sys::window().unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    cb2.as_ref().unchecked_ref(), delay + 600
                                );
                        }
                    }
                },
                1 => {
                    // Word stagger
                    for (i, (set_op, set_y)) in word_w_ops.iter().zip(word_w_ys.iter()).enumerate() {
                        let set_op = *set_op;
                        let set_y = *set_y;
                        let total = word_w_ops.len();
                        let delay = (i as i32) * 100;
                        let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                            let ty = Rc::new(RefCell::new(
                                Tween::new(25.0_f32, 0.0).duration(0.6).easing(Easing::EaseOutBack).build()
                            ));
                            let to = Rc::new(RefCell::new(
                                Tween::new(0.0_f32, 1.0).duration(0.5).easing(Easing::EaseOutCubic).build()
                            ));
                            let tty = ty.clone();
                            let tto = to.clone();
                            animation::start_raf_loop(move |dt| {
                                let mut y = tty.borrow_mut();
                                let mut o = tto.borrow_mut();
                                if *y.state() != TweenState::Completed {
                                    y.update(dt);
                                    o.update(dt);
                                    set_y.set(y.value());
                                    set_op.set(o.value());
                                }
                            });
                        });
                        let _ = web_sys::window().unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                cb.as_ref().unchecked_ref(), delay
                            );
                        if i == total - 1 {
                            let cb2 = wasm_bindgen::closure::Closure::once_into_js(move || {
                                set_playing.set(false);
                            });
                            let _ = web_sys::window().unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    cb2.as_ref().unchecked_ref(), delay + 700
                                );
                        }
                    }
                },
                _ => {
                    // Line reveal
                    animation::tween_signal(40.0, 0.0, 0.8, Easing::EaseOutExpo, set_line_y);
                    animation::tween_signal(0.0, 1.0, 0.6, Easing::EaseOutCubic, set_line_op);
                    let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                        set_playing.set(false);
                    });
                    let _ = web_sys::window().unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.as_ref().unchecked_ref(), 1000
                        );
                },
            }
        }
    };

    // Build char views
    let char_views: Vec<_> = chars.iter().enumerate().map(|(i, ch)| {
        let (op, _) = char_ops[i];
        let (y, _) = char_ys[i];
        let ch = if *ch == ' ' { "\u{00A0}".to_string() } else { ch.to_string() };
        view! {
            <span style=move || format!(
                "display: inline-block; opacity: {}; transform: translateY({}px); transition: none; font-size: 2rem;",
                op.get(), y.get()
            )>
                {ch.clone()}
            </span>
        }
    }).collect();

    let word_views: Vec<_> = words.iter().enumerate().map(|(i, word)| {
        let (op, _) = word_ops[i];
        let (y, _) = word_ys[i];
        let word = *word;
        view! {
            <span style=move || format!(
                "display: inline-block; opacity: {}; transform: translateY({}px); margin-right: 0.4em; transition: none; font-size: 2rem;",
                op.get(), y.get()
            )>
                {word}
            </span>
        }
    }).collect();

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                <h3 style="font-size: 1.1rem; font-weight: 600;">"Split Text Animation"</h3>
                <button class="btn-primary" style="padding: 0.5rem 1.5rem; font-size: 0.85rem;" on:click=play>
                    {move || if playing.get() { "Animating..." } else { "▶ Play" }}
                </button>
            </div>

            // Mode selector
            <div style="display: flex; gap: 0.5rem; margin-bottom: 2rem;">
                <button
                    class=move || if mode.get() == 0 { "easing-option active" } else { "easing-option" }
                    on:click=move |_| set_mode.set(0)
                >"By Character"</button>
                <button
                    class=move || if mode.get() == 1 { "easing-option active" } else { "easing-option" }
                    on:click=move |_| set_mode.set(1)
                >"By Word"</button>
                <button
                    class=move || if mode.get() == 2 { "easing-option active" } else { "easing-option" }
                    on:click=move |_| set_mode.set(2)
                >"By Line"</button>
            </div>

            // Preview area
            <div style="min-height: 120px; display: flex; align-items: center; justify-content: center; border-radius: 12px; background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.05); padding: 2rem; overflow: hidden;">
                {move || match mode.get() {
                    0 => {
                        // Char mode
                        view! {
                            <div style="font-family: 'Space Grotesk', sans-serif; font-weight: 600; display: flex; flex-wrap: wrap; justify-content: center;">
                                {char_views.clone()}
                            </div>
                        }.into_any()
                    },
                    1 => {
                        // Word mode
                        view! {
                            <div style="font-family: 'Space Grotesk', sans-serif; font-weight: 600; display: flex; flex-wrap: wrap; justify-content: center;">
                                {word_views.clone()}
                            </div>
                        }.into_any()
                    },
                    _ => {
                        // Line mode
                        view! {
                            <div style=move || format!(
                                "font-family: 'Space Grotesk', sans-serif; font-weight: 600; font-size: 2rem; \
                                 opacity: {}; transform: translateY({}px); transition: none;",
                                line_op.get(), line_y.get()
                            )>
                                {demo_text}
                            </div>
                        }.into_any()
                    },
                }}
            </div>

            <p style="margin-top: 1rem; font-size: 0.75rem; color: rgba(255,255,255,0.3); font-family: 'JetBrains Mono', monospace; text-align: center;">
                "SplitText + stagger() + Tween — GSAP-style text animation in Rust"
            </p>
        </div>
    }
}
