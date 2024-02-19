use leptos::html::Div;
use leptos::Children;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_element_size, UseElementSizeReturn};
use log::debug;
#[component]
pub fn TioCard(children: Children, resize: bool, size: RwSignal<Position>) -> impl IntoView {
    let cardEl = create_node_ref::<Div>();
    let UseElementSizeReturn { width, height } = use_element_size(cardEl);

    let resize_effect = create_effect(move |_| {
        debug!("Resize effect");
        if (width.get() != size.get().x || height.get() != size.get().y) {
            size.set(Position {
                x: width.get(),
                y: height.get(),
            });
        }
        debug!("{}", width.get().to_string());
    });
    let resize_string = if resize {
        "resize: both; overflow: hidden;"
    } else {
        ""
    };

    view! {
        <div
            node_ref=cardEl
            style=format!(
                "background-color: #F5F5F5; margin: 2px; border-radius: 4px; border:2px solid black;{};background-color: #EEF7F4",
                resize_string,
            )
        >

            {children()}
        </div>
    }
}
