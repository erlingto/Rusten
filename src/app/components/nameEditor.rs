use crate::app::{components::styling::TEXTINPUT, tio::tioButton::TioButton};
use leptos::{
    html::{Div, Input},
    *,
};
use leptos_use::{on_click_outside, use_element_hover};
#[component]
pub fn NameEditor(name: RwSignal<String>, editable: RwSignal<bool>) -> impl IntoView {
    let el = create_node_ref::<Input>();
    let is_hovered = use_element_hover(el);
    on_click_outside(el, move |event| editable.set(false));

    let inputStyle = Signal::derive(move || {
        let mut color = "#088F8F";
        if editable.get() || is_hovered.get() {
            color = "#0bb8b8";
        }
        format!("border: 0px solid #ccc; font-size: 16px; padding: 0; margin: 0; width: 100%; color: black; background-color: {}", color)
    });

    view! {
        <div style="margin:0">
            <div style="display: inline-flex;">
                <input
                    node_ref=el
                    style=inputStyle
                    type="text"
                    size="500"
                    disabled=move || !(is_hovered.get() || editable.get())
                    prop:value=name.get()
                    on:change=move |e| name.set(event_target_value(&e))
                    on:click=move |_| editable.set(true)
                />
                <p style="position: relative; margin: 0; padding: 1px; padding-left: 10px; background-color: #088F8F">
                    "🤚"
                </p>
            </div>
            <hr style="margin: 0; border-top: 1px solid #bbb;"/>
        </div>
    }
}
