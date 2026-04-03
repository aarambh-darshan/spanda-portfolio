mod animation;
mod app;
mod cursor;
mod sections;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
