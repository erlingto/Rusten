use leptos::*;
use log::debug;

use crate::app::{
    components::{attributeEditor::AttributeEditor, styling::TEXTINPUT},
    structs::MoveBoxAttribute::MoveBoxAttribute,
    tio::tioButton::TioButton,
};
#[component]
pub fn AttributesEditor(
    id: String,
    name: RwSignal<String>,
    attributes: RwSignal<Vec<MoveBoxAttribute>>,
) -> impl IntoView {
    view! {
        <div style="display: inline-flex; margin: 1px">
            <p style="margin: 0">"Name:  "</p>
            <input
                style=TEXTINPUT.to_string()
                size="500"
                type="text"
                prop:value=name.get()
                on:change=move |e| name.set(event_target_value(&e))
            />
        </div>
        <hr style="border-top: 3px solid #bbb; margin:0;"/>

        <For each=attributes key=|state| state.key.clone() let:child>
            <AttributeEditor attribute=child.value/>
        </For>
        <TioButton
            onClick=move || {
                debug!("Adding new attribute");
                let mut newAtt = attributes.get();
                debug!("newAtt: {:?}", newAtt);
                newAtt
                    .push(MoveBoxAttribute {
                        key: format!("{}, {}", id, attributes.get().len()),
                        value: create_rw_signal(String::from("")),
                    });
                attributes.set(newAtt);
            }

            style="position:relative; bottom:2px; font-size: 12px; margin:0; padding: 0; width: 20px; height: 20px;"
                .to_string()
            text=Signal::derive(move || "➕".to_string())
        />
    }
}
