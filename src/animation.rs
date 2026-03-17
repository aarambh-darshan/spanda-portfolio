//! Spanda ↔ Leptos animation bridge.

use std::cell::RefCell;
use std::rc::Rc;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use spanda::{Tween, Easing, Spring, SpringConfig};
use spanda::traits::Update as _;
use spanda::tween::TweenState;

/// Request a single `requestAnimationFrame` callback.
pub fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) -> i32 {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap()
}

/// Start a self-scheduling rAF loop that calls `callback(dt_seconds)` each frame.
pub fn start_raf_loop(mut callback: impl FnMut(f32) + 'static) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let last_ts: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(None));

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        let dt = {
            let mut last = last_ts.borrow_mut();
            let dt = match *last {
                Some(prev) => ((timestamp - prev) / 1000.0) as f32,
                None => 0.0,
            };
            *last = Some(timestamp);
            dt.max(0.0).min(0.1)
        };

        callback(dt);

        if let Some(ref closure) = *f.borrow() {
            let _ = request_animation_frame(closure);
        }
    }) as Box<dyn FnMut(f64)>));

    {
        let guard = g.borrow();
        if let Some(ref closure) = *guard {
            let _ = request_animation_frame(closure);
        }
    }
}

/// Start a cancellable rAF loop using an animation ID pattern.
/// The loop will stop if `current_id` no longer equals `my_id`.
pub fn start_cancellable_raf_loop(
    my_id: u32,
    current_id: Rc<std::cell::Cell<u32>>,
    mut callback: impl FnMut(f32) + 'static,
) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let last_ts: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(None));

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        // Stop if a new animation has been started
        if current_id.get() != my_id {
            return;
        }

        let dt = {
            let mut last = last_ts.borrow_mut();
            let dt = match *last {
                Some(prev) => ((timestamp - prev) / 1000.0) as f32,
                None => 0.0,
            };
            *last = Some(timestamp);
            dt.max(0.0).min(0.1)
        };

        callback(dt);

        if current_id.get() == my_id {
            if let Some(ref closure) = *f.borrow() {
                let _ = request_animation_frame(closure);
            }
        }
    }) as Box<dyn FnMut(f64)>));

    {
        let guard = g.borrow();
        if let Some(ref closure) = *guard {
            let _ = request_animation_frame(closure);
        }
    }
}

/// Run a simple Tween driving a signal, auto-cleaning up when done.
pub fn tween_signal(
    from: f32,
    to: f32,
    duration: f32,
    easing: Easing,
    signal: WriteSignal<f32>,
) {
    let tween = Rc::new(RefCell::new(
        Tween::new(from, to)
            .duration(duration)
            .easing(easing)
            .build(),
    ));

    let tween_clone = tween.clone();
    start_raf_loop(move |dt| {
        let mut t = tween_clone.borrow_mut();
        if *t.state() != TweenState::Completed {
            t.update(dt);
            signal.set(t.value());
        }
    });
}

/// Run a spring animation driving a signal.
#[allow(dead_code)]
pub fn spring_signal(
    config: SpringConfig,
    target: f32,
    signal: WriteSignal<f32>,
) {
    let spring = Rc::new(RefCell::new(Spring::new(config)));

    let spring_clone = spring.clone();
    {
        let mut s = spring_clone.borrow_mut();
        s.set_target(target);
    }

    start_raf_loop(move |dt| {
        let mut s = spring_clone.borrow_mut();
        if !s.is_settled() {
            s.update(dt);
            signal.set(s.position());
        }
    });
}
