use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use spanda::{Tween, Easing};
use spanda::traits::Update as _;
use spanda::tween::TweenState;
use crate::animation;

#[component]
pub fn Hero() -> impl IntoView {
    // Split text animation signals — one per letter
    let title_text = "SPANDA";
    let subtitle_words = [
        "A", "general-purpose", "animation", "library", "for", "Rust", "—",
        "tweening,", "keyframes,", "timelines,", "springs", "&", "physics.",
        "Anywhere", "Rust", "runs.",
    ];

    let letter_opacities: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..title_text.len()).map(|_| signal(0.0_f32)).collect();
    let letter_ys: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..title_text.len()).map(|_| signal(80.0_f32)).collect();

    let word_opacities: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..subtitle_words.len()).map(|_| signal(0.0_f32)).collect();
    let word_ys: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..subtitle_words.len()).map(|_| signal(20.0_f32)).collect();

    let (badge_opacity, set_badge_opacity) = signal(0.0_f32);
    let (cta_opacity, set_cta_opacity) = signal(0.0_f32);

    // After splash screen fades (3.3s), animate hero text in with stagger
    Effect::new({
        let letter_ys = letter_ys.iter().map(|(_, w)| *w).collect::<Vec<_>>();
        let letter_ops = letter_opacities.iter().map(|(_, w)| *w).collect::<Vec<_>>();
        let word_ys = word_ys.iter().map(|(_, w)| *w).collect::<Vec<_>>();
        let word_ops = word_opacities.iter().map(|(_, w)| *w).collect::<Vec<_>>();
        move || {
            // Stagger title letters at 80ms intervals, starting at 3.3s
            for (i, (set_y, set_op)) in letter_ys.iter().zip(letter_ops.iter()).enumerate() {
                let set_y = *set_y;
                let set_op = *set_op;
                let delay = 3300 + (i as i32) * 80;
                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                    // Animate Y from 80 → 0
                    let tween_y = Rc::new(RefCell::new(
                        Tween::new(80.0_f32, 0.0).duration(0.7).easing(Easing::EaseOutExpo).build()
                    ));
                    let tween_op = Rc::new(RefCell::new(
                        Tween::new(0.0_f32, 1.0).duration(0.6).easing(Easing::EaseOutCubic).build()
                    ));
                    let ty = tween_y.clone();
                    let to = tween_op.clone();
                    animation::start_raf_loop(move |dt| {
                        let mut t = ty.borrow_mut();
                        let mut o = to.borrow_mut();
                        if *t.state() != TweenState::Completed {
                            t.update(dt);
                            o.update(dt);
                            set_y.set(t.value());
                            set_op.set(o.value());
                        }
                    });
                });
                let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(), delay,
                );
            }

            // Stagger subtitle words at 40ms intervals, starting later
            for (i, (set_y, set_op)) in word_ys.iter().zip(word_ops.iter()).enumerate() {
                let set_y = *set_y;
                let set_op = *set_op;
                let delay = 3900 + (i as i32) * 40;
                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                    let tween_y = Rc::new(RefCell::new(
                        Tween::new(20.0_f32, 0.0).duration(0.6).easing(Easing::EaseOutCubic).build()
                    ));
                    let tween_op = Rc::new(RefCell::new(
                        Tween::new(0.0_f32, 1.0).duration(0.5).easing(Easing::EaseOutCubic).build()
                    ));
                    let ty = tween_y.clone();
                    let to = tween_op.clone();
                    animation::start_raf_loop(move |dt| {
                        let mut t = ty.borrow_mut();
                        let mut o = to.borrow_mut();
                        if *t.state() != TweenState::Completed {
                            t.update(dt);
                            o.update(dt);
                            set_y.set(t.value());
                            set_op.set(o.value());
                        }
                    });
                });
                let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(), delay,
                );
            }

            // Badge at 4.8s
            {
                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                    animation::tween_signal(0.0, 1.0, 0.8, Easing::EaseOutCubic, set_badge_opacity);
                });
                let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(), 4800,
                );
            }
            // CTA at 5.0s
            {
                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                    animation::tween_signal(0.0, 1.0, 0.8, Easing::EaseOutCubic, set_cta_opacity);
                });
                let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(), 5000,
                );
            }
        }
    });

    // Build letter views
    let title_chars: Vec<char> = title_text.chars().collect();
    let letter_views: Vec<_> = title_chars.into_iter().enumerate().map(|(i, ch)| {
        let (op, _) = letter_opacities[i];
        let (y, _) = letter_ys[i];
        let ch_str = String::from(ch);
        view! {
            <span style=move || format!(
                "display: inline-block; opacity: {}; transform: translateY({}px); transition: none;",
                op.get(), y.get()
            )>
                {ch_str.clone()}
            </span>
        }
    }).collect();

    // Build word views
    let word_views: Vec<_> = subtitle_words.iter().enumerate().map(|(i, word)| {
        let (op, _) = word_opacities[i];
        let (y, _) = word_ys[i];
        let word = *word;
        view! {
            <span style=move || format!(
                "display: inline-block; opacity: {}; transform: translateY({}px); margin-right: 0.35em; transition: none;",
                op.get(), y.get()
            )>
                {word}
            </span>
        }
    }).collect();

    view! {
        <section class="hero-section" id="hero">
            <div class="hero-bg-mesh" />
            <div class="orb orb-1" />
            <div class="orb orb-2" />
            <div class="orb orb-3" />

            <div style="position: relative; z-index: 1; display: flex; flex-direction: column; align-items: center; padding: 0 2rem;">
                <h1 class="hero-title" style="overflow: hidden;">
                    {letter_views}
                </h1>

                <p class="hero-subtitle" style="opacity: 1; animation: none; display: flex; flex-wrap: wrap; justify-content: center;">
                    {word_views}
                </p>

                <div class="hero-badge" style=move || format!(
                    "opacity: {}; animation: none; transform: translateY({}px);",
                    badge_opacity.get(),
                    (1.0 - badge_opacity.get()) * 15.0
                )>
                    <span>"🦀"</span>
                    <span>"v0.8  ·  no_std  ·  zero unsafe"</span>
                </div>

                <div class="hero-cta-group" style=move || format!(
                    "opacity: {}; animation: none; transform: translateY({}px);",
                    cta_opacity.get(),
                    (1.0 - cta_opacity.get()) * 15.0
                )>
                    <a href="https://github.com/aarambh-darshan/spanda" target="_blank" class="btn-primary">
                        "⭐ Star on GitHub"
                    </a>
                    <a href="https://docs.rs/spanda" target="_blank" class="btn-secondary">
                        "📖 Read the Docs"
                    </a>
                </div>
            </div>

            <div class="scroll-indicator">
                <span>"Scroll"</span>
                <div class="scroll-indicator-line" />
            </div>
        </section>
    }
}
