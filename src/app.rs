use crate::app::components::canvas::Canvas;
pub mod components;
pub mod structs;
use leptos::*;
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Canvas width=80 height=85/>
        </div>
    }
}
