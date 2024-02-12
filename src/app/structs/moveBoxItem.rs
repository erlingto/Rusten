use leptos::RwSignal;
use leptos_use::core::Position;

use super::MoveBoxAttribute::MoveBoxAttribute;

#[derive(Debug, Clone)]
pub struct MoveBoxItem {
    pub key: String,
    pub value: RwSignal<String>,
    pub position: RwSignal<Position>,
    pub attributes: RwSignal<Vec<MoveBoxAttribute>>,
}
