use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::animation;
use spanda::Easing;

#[component]
pub fn Splash() -> impl IntoView {
    let (progress, set_progress) = signal(0.0_f32);
    let (visible, set_visible) = signal(true);
    let (fade_out, set_fade_out) = signal(false);

    Effect::new(move || {
        animation::tween_signal(
            0.0,
            100.0,
            2.2,
            Easing::EaseOutExpo,
            set_progress,
        );

        let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
            set_fade_out.set(true);
            let cb2 = wasm_bindgen::closure::Closure::once_into_js(move || {
                set_visible.set(false);
            });
            let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                cb2.as_ref().unchecked_ref(),
                800,
            );
        });
        let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            2500,
        );
    });

    move || {
        if visible.get() {
            Some(view! {
                <div class=move || {
                    if fade_out.get() {
                        "splash-screen fade-out"
                    } else {
                        "splash-screen"
                    }
                }>
                    <div class="splash-logo">"spanda"</div>
                    <div class="splash-sanskrit">"स्पन्द"</div>
                    <div class="splash-progress-track">
                        <div
                            class="splash-progress-bar"
                            style:width=move || format!("{}%", progress.get())
                        />
                    </div>
                </div>
            })
        } else {
            None
        }
    }
}
