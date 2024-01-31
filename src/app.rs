use crate::app::components::button::Button;
use crate::app::components::canvas::Canvas;
pub mod components;
use leptos::*;
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Canvas/>
        </div>
    }
}
