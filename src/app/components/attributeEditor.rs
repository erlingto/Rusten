use leptos::{html::Div, *};
use leptos_use::{on_click_outside, use_element_hover};

use crate::app::{components::styling::TEXTINPUT, tio::tioButton::TioButton};
#[component]
pub fn AttributeEditor(attribute: RwSignal<String>) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let active = create_rw_signal(false);
    let is_hovered = use_element_hover(el);
    on_click_outside(el, move |event| active.set(false));

    view! {
        <div node_ref=el style="margin:0" on:click=move |_| active.set(true)>
            <div style="display: inline-flex;">
                <p style="margin: 0">"➖ "</p>
                <input
                    style=TEXTINPUT.to_string()
                    type="text"
                    size="500"
                    disabled=move || !(is_hovered.get() || active.get())
                    prop:value=attribute.get()
                    on:change=move |e| attribute.set(event_target_value(&e))
                />
            </div>
            <hr style="margin: 0; border-top: 1px solid #bbb;"/>
        </div>
    }
}
