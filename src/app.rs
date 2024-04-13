use crate::app::components::mermaidEditor::MermaidEditor;
pub mod components;
pub mod helpers;
pub mod structs;
pub mod tio;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div style="overflow: hidden;">
            <MermaidEditor/>
        </div>
    }
}
