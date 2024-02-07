use leptos::*;
use leptos_use::core::Position;

#[derive(Debug, Clone)]
pub struct MoveBoxItem {
    pub key: String,
    pub value: String,
    pub position: RwSignal<Position>,
}

#[derive(Debug, Clone)]
pub struct ConnectionItem {
    pub key: String,
    pub from: RwSignal<MoveBoxItem>,
    pub to: RwSignal<MoveBoxItem>,
}
