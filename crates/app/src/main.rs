use leptos::{mount_to_body, view};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <p>"Hello, World!"</p> })
}
