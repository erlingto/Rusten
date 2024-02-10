use leptos::*;
#[component]
pub fn TioCard(children: Children) -> impl IntoView {
    view! {
        <div style=format!(
            "background-color: #F5F5F5; margin: 2px; border-radius: 4px; border:2px solid black",
        )>{children()}</div>
    }
}
