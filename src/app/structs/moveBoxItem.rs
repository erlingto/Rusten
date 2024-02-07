use leptos::RwSignal;
use leptos_use::core::Position;

#[derive(Debug, Clone)]
pub struct MoveBoxItem {
    pub key: String,
    pub value: RwSignal<String>,
    pub position: RwSignal<Position>,
}
