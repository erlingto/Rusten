use leptos::RwSignal;
use leptos_use::core::Position;

use super::MoveBoxAttribute::MoveBoxAttribute;

#[derive(Debug, Clone)]
pub struct MoveBoxItem {
    pub key: String,
    pub should_render: RwSignal<bool>,
    pub value: RwSignal<String>,
    pub position: RwSignal<Position>,
    pub realPosition: RwSignal<Position>,
    pub isDragging: RwSignal<bool>,
    pub attributes: RwSignal<Vec<MoveBoxAttribute>>,
    pub size: RwSignal<Position>,
    pub mounted: RwSignal<bool>,
}
