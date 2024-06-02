use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex flex-row h-screen w-screen font-lato">
            <div class="w-64 bg-gray-100 text-gray-700 p-4 border-r border-gray-300 flex flex-col justify-between">
                <div class="flex flex-col gap-y-2">
                    <a href="#" class="text-slate-600 hover:text-slate-800">
                        <i class="ph ph-file"></i>
                        Page 1
                    </a>
                    <a href="#" class="text-slate-600 hover:text-slate-800">
                        <i class="ph ph-file"></i>
                        Page 2
                    </a>
                    <a href="#" class="text-slate-600 hover:text-slate-800">
                        <i class="ph ph-file"></i>
                        Page 3
                    </a>
                </div>
                <div class="flex flex-row gap-x-2 text-2xl align-end">
                    <a href="#" class="text-slate-600 hover:text-slate-800">
                        <i class="ph ph-user"></i>
                    </a>
                    <a href="#" class="text-slate-600 hover:text-slate-800">
                        <i class="ph ph-gear"></i>
                    </a>
                    <a href="#" class="text-slate-600 hover:text-slate-800">
                        <i class="ph ph-question-mark"></i>
                    </a>
                </div>
            </div>
            <div class="flex-1 bg-gray-50 p-2">
                <p>
                    <i class="ph ph-atom"></i>
                    Hello, World!
                </p>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
