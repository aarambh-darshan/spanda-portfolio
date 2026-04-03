use crate::animation;
use leptos::prelude::*;
use spanda::traits::Update as _;
use spanda::tween::TweenState;
use spanda::{Easing, Tween};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

#[component]
pub fn Hero() -> impl IntoView {
    // Split text animation signals — one per letter
    let title_text = "SPANDA";
    let subtitle_words = [
        "A",
        "general-purpose",
        "animation",
        "library",
        "for",
        "Rust",
        "—",
        "tweening,",
        "keyframes,",
        "timelines,",
        "springs",
        "&",
        "physics.",
        "Anywhere",
        "Rust",
        "runs.",
    ];

    let letter_opacities: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..title_text.len()).map(|_| signal(0.0_f32)).collect();
    let letter_ys: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..title_text.len()).map(|_| signal(80.0_f32)).collect();

    let word_opacities: Vec<(ReadSignal<f32>, WriteSignal<f32>)> =
        (0..subtitle_words.len()).map(|_| signal(0.0_f32)).collect();
    let word_ys: Vec<(ReadSignal<f32>, WriteSignal<f32>)> = (0..subtitle_words.len())
        .map(|_| signal(20.0_f32))
        .collect();

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
                        Tween::new(80.0_f32, 0.0)
                            .duration(0.7)
                            .easing(Easing::EaseOutExpo)
                            .build(),
                    ));
                    let tween_op = Rc::new(RefCell::new(
                        Tween::new(0.0_f32, 1.0)
                            .duration(0.6)
                            .easing(Easing::EaseOutCubic)
                            .build(),
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
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        delay,
                    );
            }

            // Stagger subtitle words at 40ms intervals, starting later
            for (i, (set_y, set_op)) in word_ys.iter().zip(word_ops.iter()).enumerate() {
                let set_y = *set_y;
                let set_op = *set_op;
                let delay = 3900 + (i as i32) * 40;
                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                    let tween_y = Rc::new(RefCell::new(
                        Tween::new(20.0_f32, 0.0)
                            .duration(0.6)
                            .easing(Easing::EaseOutCubic)
                            .build(),
                    ));
                    let tween_op = Rc::new(RefCell::new(
                        Tween::new(0.0_f32, 1.0)
                            .duration(0.5)
                            .easing(Easing::EaseOutCubic)
                            .build(),
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
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        delay,
                    );
            }

            // Badge at 4.8s
            {
                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                    animation::tween_signal(0.0, 1.0, 0.8, Easing::EaseOutCubic, set_badge_opacity);
                });
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        4800,
                    );
            }
            // CTA at 5.0s
            {
                let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                    animation::tween_signal(0.0, 1.0, 0.8, Easing::EaseOutCubic, set_cta_opacity);
                });
                let _ = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        5000,
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
        <section class="min-h-screen relative flex flex-col lg:flex-row items-center justify-between px-[5%] overflow-hidden bg-bg" id="hero">
            <div class="absolute inset-0 pointer-events-none opacity-[0.03]" style="background-image: url('data:image/svg+xml;utf8,%3Csvg viewBox=%220 0 200 200%22 xmlns=%22http://www.w3.org/2000/svg%22%3E%3Cfilter id=%22noiseFilter%22%3E%3CfeTurbulence type=%22fractalNoise%22 baseFrequency=%220.65%22 numOctaves=%223%22 stitchTiles=%22stitch%22/%3E%3C/filter%3E%3Crect width=%22100%25%22 height=%22100%25%22 filter=%22url(%23noiseFilter)%22/%3E%3C/svg%3E');"></div>

            <div class="relative z-10 flex w-full max-w-[1300px] mx-auto items-center">

                <div class="flex-1 flex flex-col items-start pt-[20vh] pb-[10vh] lg:py-0 relative">
                    <h1 class="font-instrument italic text-[14vw] lg:text-[11vw] leading-none overflow-hidden max-w-[800px] text-ink" style="letter-spacing: -0.02em;">
                        {letter_views}
                    </h1>

                    <p class="font-syne text-ink-muted text-[1rem] lg:text-[1.3rem] mt-6 max-w-[600px] font-medium leading-relaxed flex flex-wrap justify-start" style="opacity: 1; animation: none;">
                        {word_views}
                    </p>

                    <div class="hidden lg:block absolute bottom-[-10vh] left-[-5vw] origin-bottom-left -rotate-90 font-mono text-[4rem] text-ink-muted whitespace-nowrap" style=move || format!(
                        "opacity: {}; transform: translateY({}px) rotate(-90deg);",
                        badge_opacity.get() * 0.15,
                        (1.0 - badge_opacity.get()) * 15.0
                    )>
                        "v0.9.2 / SPANDA"
                    </div>

                    <div class="flex flex-col sm:flex-row gap-4 mt-12" style=move || format!(
                        "opacity: {}; animation: none; transform: translateY({}px);",
                        cta_opacity.get(),
                        (1.0 - cta_opacity.get()) * 15.0
                    )>
                        <a href="https://github.com/aarambh-darshan/spanda" target="_blank" class="bg-black text-accent font-mono uppercase px-8 py-4 border-2 border-black inline-flex items-center gap-2 hover:bg-accent-dark hover:text-black transition-colors" style="cursor: none">
                            "⭐ Star on GitHub"
                        </a>
                        <a href="https://docs.rs/spanda" target="_blank" class="bg-transparent text-black font-mono uppercase px-8 py-4 border-2 border-border inline-flex items-center gap-2 hover:bg-black hover:text-accent transition-colors" style="cursor: none">
                            "📖 Read the Docs"
                        </a>
                    </div>
                </div>

                <div class="flex-[0.8] relative h-[500px] hidden lg:flex items-center justify-center" style="perspective: 1000px;">
                    <div class="w-[300px] h-[400px] bg-bg border-[4px] border-border -rotate-12 flex flex-col p-8 gap-4 absolute z-20" style="box-shadow: 20px 20px 0px rgba(10,10,10,0.1)">
                        <div class="w-full h-[2px] bg-ink"></div>
                        <div class="w-2/3 h-[2px] bg-ink"></div>
                        <div class="w-12 h-12 bg-accent mt-auto self-end border-[4px] border-border"></div>
                    </div>
                    <div class="w-[250px] h-[350px] bg-code-bg border-[4px] border-accent translate-x-[60px] translate-y-[40px] absolute z-10 flex flex-col p-6 gap-3">
                        <div class="w-full h-3 bg-accent"></div>
                        <div class="w-3/4 h-3 bg-accent opacity-60"></div>
                    </div>
                </div>

            </div>
        </section>
    }
}
