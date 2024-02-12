use leptos::RwSignal;

#[derive(Debug, Clone)]
pub struct MoveBoxAttribute {
    pub key: String,
    pub value: RwSignal<String>,
}
