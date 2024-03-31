use leptos::*;

use crate::app::{
    components::attributeEditor::AttributeEditor, structs::MoveBoxAttribute::MoveBoxAttribute,
    tio::tioButton::TioButton,
};
#[component]
pub fn AttributesEditor(id: String, attributes: RwSignal<Vec<MoveBoxAttribute>>) -> impl IntoView {
    let count = create_rw_signal(0);
    let removeAttribute = move |key: String| {
        let mut newAtt = attributes.get();
        newAtt.retain(|x| x.key != key);
        attributes.set(newAtt);
    };
    view! {
        <hr style="border-top: 3px solid #bbb; margin:0;"/>

        <For each=attributes key=|state| state.key.clone() let:child>
            <AttributeEditor
                attribute=child.value
                remove=move || removeAttribute(child.key.clone())
            />
        </For>
        <TioButton
            on_click=move || {
                let mut newAtt = attributes.get();
                newAtt
                    .push(MoveBoxAttribute {
                        key: format!("{}, {}", id, count.get()),
                        value: create_rw_signal(String::from("")),
                    });
                count.set(count.get() + 1);
                attributes.set(newAtt);
            }

            style="position:relative; bottom:2px; font-size: 12px; margin:0; padding: 0; width: 20px; height: 20px;"
                .to_string()
            text=Signal::derive(move || "➕".to_string())
        />
    }
}
