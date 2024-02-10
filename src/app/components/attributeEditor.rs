use leptos::*;

use crate::app::tio::tioButton::TioButton;
#[component]
pub fn AttributeEditor(
    id: String,
    name: RwSignal<String>,
    attributes: RwSignal<Vec<RwSignal<String>>>,
) -> impl IntoView {
    view! {
        <p style="width: 80%; margin: 0">"Edit Name"</p>
        <input
            style="width: 80%; margin: 0"
            type="text"
            prop:value=name.get()
            on:change=move |e| name.set(event_target_value(&e))
        />
        <For each=attributes key=|state| state.get() let:child>
            <input
                style="width: 80%; margin: 0"
                type="text"
                prop:value=child.get()
                on:change=move |e| child.set(event_target_value(&e))
            />
        </For>
        <TioButton
            onClick=move || {
                let mut newAtt = attributes.get();
                newAtt.push(create_rw_signal(String::from("")));
                attributes.set(newAtt);
            }

            style="font-size: 12px; margin:0; padding: 0; width: 80%; height: 20px;".to_string()
            text=Signal::derive(move || "➕".to_string())
        />
    }
}
