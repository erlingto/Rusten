use leptos::{html::Div, *};
use leptos_use::{on_click_outside, use_element_hover};

use crate::app::components::styling::TEXTINPUT;
#[component]
pub fn AttributeEditor<F: Fn() -> () + 'static>(
    attribute: RwSignal<String>,
    remove: F,
) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let active = create_rw_signal(false);
    let is_hovered = use_element_hover(el);
    let _ = on_click_outside(el, move |_| active.set(false));

    view! {
        <div node_ref=el style="margin:0" on:click=move |_| active.set(true)>
            <div style="display: inline-flex;">
                <div style="margin: 0; cursor: pointer" on:click=move |_| remove()>
                    "➖ "
                </div>
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
