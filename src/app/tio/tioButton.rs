use leptos::{html::Button, *};
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};
#[component]
pub fn TioButton<F: Fn() + 'static>(
    on_click: F,
    text: Signal<String>,
    #[prop(default = Signal::derive(move || "".to_string()) )] style: Signal<String>,
    #[prop(default = Signal::derive(move || false) )] disabled: Signal<bool>,
) -> impl IntoView {
    let el = create_node_ref::<Button>();
    let is_hovered = use_element_hover_with_options(el, UseElementHoverOptions::default());
    let outStylestyle = move || {
        if disabled.get() {
            return format!("background-color: #e0e0e0; color: #a8a8a8; cursor: not-allowed; padding: 5px; padding-left: 15px; padding-right:15px; border: 1px solid; border-radius: 5px; width: 5; font-size: 20px;{} ", style.get());
        }
        if is_hovered.get() {
            format!("background-color: #92BFA3 ; color: black; padding: 5px; padding-left: 15px; padding-right:15px; border: 1px solid; border-radius: 5px; width: 5; font-size: 20px;{} ", style.get())
        } else {
            format!("background-color: #BCF9D3; color: black; padding: 5px; padding-left: 15px; padding-right:15px; border: 1px solid; border-radius: 5px; width: 5; font-size: 20px;{} ", style.get())
        }
    };
    view! {
        <button
            disabled=disabled
            node_ref=el
            focusable=true
            style=outStylestyle
            on:click=move |_| {
                on_click();
            }
        >

            {text}
        </button>
    }
}
