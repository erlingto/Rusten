use leptos::RwSignal;

use super::moveBoxItem::MoveBoxItem;

#[derive(Debug, Clone)]
pub struct ConnectionItem {
    pub key: String,
    pub from: RwSignal<MoveBoxItem>,
    pub to: RwSignal<MoveBoxItem>,
}
