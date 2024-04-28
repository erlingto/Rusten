use crate::app::components::mermaidEditor::MermaidEditor;
pub mod components;
pub mod helpers;
pub mod structs;
pub mod tio;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <MermaidEditor/>
        </Router>
    }
}
