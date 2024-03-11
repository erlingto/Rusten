use crate::app::components::canvas::Canvas;
pub mod components;
pub mod helpers;
pub mod structs;
pub mod tio;
use leptos::*;
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Canvas width=80 height=95/>
        </div>
    }
}
