use leptos::{html::Input, *};
use leptos_use::{on_click_outside, use_element_hover};
#[component]
pub fn NameEditor(name: RwSignal<String>, editable: RwSignal<bool>) -> impl IntoView {
    let el = create_node_ref::<Input>();
    let is_hovered = use_element_hover(el);
    let _ = on_click_outside(el, move |_| editable.set(false));
    let scale = use_context::<RwSignal<f64>>().expect("there to be a `count` signal provided");

    let inputStyle = Signal::derive(move || {
        let mut color = "#088F8F";
        if editable.get() || is_hovered.get() {
            color = "#0bb8b8";
        }
        format!("border: 0px solid #ccc; font-size: {}px; padding: 0; margin: 0; width: 100%; color: black; background-color: {}", 16.0 * scale.get(), color)
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
                    on:click=move |e| {
                        e.prevent_default();
                        editable.set(true)
                    }
                />

                <p style=move || {
                    format!(
                        "position: relative; margin: 0; padding: 1px; padding-left: 10px; background-color: #088F8F; font-size: {}px",
                        16.0 * scale.get(),
                    )
                }>"🤚"</p>
            </div>
        </div>
    }
}
