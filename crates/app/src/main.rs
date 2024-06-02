use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! { <p>"Hello, World!"</p> }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
