use crate::app::components::canvas::MermaidEditor;
pub mod components;
pub mod helpers;
pub mod structs;
pub mod tio;
use leptos::*;
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <MermaidEditor/>
        </div>
    }
}
