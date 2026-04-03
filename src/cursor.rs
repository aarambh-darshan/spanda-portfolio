use crate::animation;
use leptos::prelude::*;
use spanda::spring::{SpringConfig, SpringN};
use spanda::traits::Update;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

#[component]
pub fn Cursor() -> impl IntoView {
    let (cursor_x, set_cursor_x) = signal(0.0_f32);
    let (cursor_y, set_cursor_y) = signal(0.0_f32);
    let (visible, set_visible) = signal(false);
    let (ring_scale, set_ring_scale) = signal(1.0_f32);

    let ring_spring = Rc::new(RefCell::new(SpringN::new(
        SpringConfig {
            stiffness: 200.0,
            damping: 15.0,
            mass: 1.0,
            epsilon: 0.001,
        },
        [1.0_f32, 0.0],
    )));

    let ring_spring1 = ring_spring.clone();
    let ring_spring2 = ring_spring.clone();

    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let mouse_x = Rc::new(RefCell::new(0.0_f32));
        let mouse_y = Rc::new(RefCell::new(0.0_f32));
        let is_visible = Rc::new(RefCell::new(false));
        let ring_spring = ring_spring1.clone();

        let mouse_x_clone = mouse_x.clone();
        let mouse_y_clone = mouse_y.clone();
        let is_visible_clone = is_visible.clone();
        let set_visible_clone = set_visible.clone();

        let move_handler = move |ev: web_sys::MouseEvent| {
            let x = ev.client_x() as f32;
            let y = ev.client_y() as f32;
            *mouse_x_clone.borrow_mut() = x;
            *mouse_y_clone.borrow_mut() = y;
            if !*is_visible_clone.borrow() {
                *is_visible_clone.borrow_mut() = true;
                set_visible_clone.set(true);
            }
        };

        let closure = wasm_bindgen::closure::Closure::wrap(
            Box::new(move_handler) as Box<dyn FnMut(web_sys::MouseEvent)>
        );
        document
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        let ring_spring_raf = ring_spring.clone();
        animation::start_raf_loop(move |_dt| {
            let mx = *mouse_x.borrow();
            let my = *mouse_y.borrow();
            let visible = *is_visible.borrow();

            if visible {
                let cx = cursor_x.get();
                let cy = cursor_y.get();
                let new_x = cx + (mx - cx) * 0.15;
                let new_y = cy + (my - cy) * 0.15;
                set_cursor_x.set(new_x);
                set_cursor_y.set(new_y);
            }

            ring_spring_raf.borrow_mut().update(_dt);
            let scale = ring_spring_raf.borrow().position();
            set_ring_scale.set(scale[0]);
        });
    });

    Effect::new(move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        let ring_spring = ring_spring2.clone();

        let add_hover_listeners = || {
            let elements =
                document.query_selector_all("a, button, .hover-target, [role=\"button\"]");
            if let Ok(nodes) = elements {
                for i in 0..nodes.length() {
                    if let Some(node) = nodes.get(i) {
                        let el = node.dyn_into::<web_sys::HtmlElement>();
                        if let Ok(el) = el {
                            let ring_spring_clone = ring_spring.clone();
                            let enter = wasm_bindgen::closure::Closure::wrap(Box::new(
                                move |_: web_sys::Event| {
                                    ring_spring_clone.borrow_mut().set_target([1.6, 0.0]);
                                },
                            )
                                as Box<dyn FnMut(web_sys::Event)>);
                            let ring_spring_clone2 = ring_spring.clone();
                            let leave = wasm_bindgen::closure::Closure::wrap(Box::new(
                                move |_: web_sys::Event| {
                                    ring_spring_clone2.borrow_mut().set_target([1.0, 0.0]);
                                },
                            )
                                as Box<dyn FnMut(web_sys::Event)>);
                            el.add_event_listener_with_callback(
                                "mouseenter",
                                enter.as_ref().unchecked_ref(),
                            )
                            .ok();
                            el.add_event_listener_with_callback(
                                "mouseleave",
                                leave.as_ref().unchecked_ref(),
                            )
                            .ok();
                            enter.forget();
                            leave.forget();
                        }
                    }
                }
            }
        };

        add_hover_listeners();
    });

    view! {
        <div
            class="cursor-container"
            style=move || {
                if !visible.get() { return String::new(); }
                format!(
                    "left: {}px; top: {}px;",
                    cursor_x.get(),
                    cursor_y.get()
                )
            }
        >
            <div class="cursor-dot"></div>
            <div
                class="cursor-ring"
                style=move || {
                    let scale = ring_scale.get();
                    let size = 50.0 * scale;
                    let offset = size / 2.0;
                    format!(
                        "width: {}px; height: {}px; top: -{}px; left: -{}px;",
                        size, size, offset, offset
                    )
                }
            ></div>
        </div>
    }
}
