use leptos::Children;
use leptos::*;
#[component]
pub fn TioCard(children: Children, resize: bool) -> impl IntoView {
    let resize_string = if resize {
        "resize: both; overflow: hidden;"
    } else {
        ""
    };

    view! {
        <div style=format!(
            "background-color: #F5F5F5; margin: 2px; border-radius: 4px; border:2px solid black;{};background-color: #EEF7F4",
            resize_string,
        )>

            {children()}
        </div>
    }
}
