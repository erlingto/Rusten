use leptos::*;
use leptos_use::core::Position;

#[derive(Debug, Clone)]
pub struct DivItem {
    pub key: String,
    pub value: i32,
    pub position: Position,
}