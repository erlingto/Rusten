use leptos::{html::Div, *};
use leptos_use::{on_click_outside, use_element_hover};

#[component]
pub fn AttributeEditor<F: Fn() + 'static>(attribute: RwSignal<String>, remove: F) -> impl IntoView {
    let scale = use_context::<RwSignal<f64>>().expect("there to be a `count` signal provided");
    let el = create_node_ref::<Div>();
    let active = create_rw_signal(false);
    let is_hovered = use_element_hover(el);
    let _ = on_click_outside(el, move |_| active.set(false));

    view! {
        <div
            node_ref=el
            style=move || format!("margin:0; height: {}px", 20.0 * scale.get())
            on:click=move |_| active.set(true)
        >
            <div style="display: inline-flex;">
                <div
                    style=move || {
                        format!(
                            "margin: 0; cursor: pointer; font-size: {}px; height: {}px",
                            12.0 * scale.get(),
                            20.0 * scale.get(),
                        )
                    }

                    on:click=move |_| remove()
                >
                    "➖ "
                </div>
                <input
                    style=move || {
                        {
                            format!(
                                "
                        width:100%; margin: 0;
                        padding: 0px;
                        border: 0px solid #ccc;
                        font-size: {}px;
                        height: {};
                        outline: none;
                        user-select: none; 
                        transition: border-color 0.3s;",
                                16.0 * scale.get(),
                                20.0 * scale.get(),
                            )
                        }
                            .to_string()
                    }

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
