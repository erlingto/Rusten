use leptos::{html::Button, *};
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};
#[component]
pub fn TioButton<F: Fn() -> () + 'static>(
    onClick: F,
    text: Signal<String>,
    style: String,
) -> impl IntoView {
    let el = create_node_ref::<Button>();
    let is_hovered = use_element_hover_with_options(el, UseElementHoverOptions::default());
    let outStylestyle = move || {
        if is_hovered.get() {
            format!("background-color: #92BFA3 ; color: black; padding: 5px; padding-left: 15px; padding-right:15px; border: 1px solid; border-radius: 5px; width: 5; font-size: 20px;{} ", style)
        } else {
            format!("background-color: #BCF9D3; color: black; padding: 5px; padding-left: 15px; padding-right:15px; border: 1px solid; border-radius: 5px; width: 5; font-size: 20px;{} ", style)
        }
    };
    view! {
        <button
            node_ref=el
            focusable=true

            style=outStylestyle
            on:click=move |_| {
                onClick();
            }
        >

            {text}
        </button>
    }
}
