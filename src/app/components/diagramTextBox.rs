use crate::app::helpers::{orderFunctions::organize_positions, parseFunctions::importDiagram};
use leptos::{html::Div, *};
use leptos_use::core::Position;
use web_sys::HtmlTextAreaElement;

use crate::app::{
    structs::{
        connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem,
        MoveBoxAttribute::MoveBoxAttribute,
    },
    tio::tioButton::TioButton,
};

#[component]
pub fn DiagramTextBox(
    connections: RwSignal<Vec<RwSignal<ConnectionItem>>>,
    items: RwSignal<Vec<RwSignal<MoveBoxItem>>>,
) -> impl IntoView {
    let (text, setText) = create_signal(String::from(""));
    let (importCount, setImportCount) = create_signal(0);
    let text_area_ref = create_node_ref::<leptos::html::Textarea>();
    let (show, setShow) = create_signal(true);

    let flickerStart = create_effect(move |_| {
        if (text.get().len() > 0) {
            setShow(false);
        }
    });

    let flicker = create_effect(move |_| {
        if (show.get() == false) {
            setShow(true);
        }
    });

    let printDiagram = move || {
        let mut connectionString = String::from(":::mermaid\n");
        connectionString.push_str("classDiagram\n");
        items.get().iter().for_each(|x| {
            connectionString.push_str(&format!("class `{}` {{ \n", x.get().value.get()));
            let att = x.get().attributes.get();
            att.iter().for_each(|y| {
                connectionString.push_str(&format!("+ {}\n", y.value.get()));
            });
            connectionString.push_str("}\n");
        });

        connections.get().iter().for_each(|x| {
            connectionString.push_str(&format!(
                "`{}` --> `{}`\n",
                x.get().from.get().value.get(),
                x.get().to.get().value.get()
            ));
        });
        connectionString
    };
    printDiagram();
    create_effect(move |_| {
        let newText = printDiagram();
        setText(newText);
    });

    let handleImport = move || {
        let (mut newItems, mut newConnections) = importDiagram(text.get(), importCount.get());
        newItems = organize_positions(newItems, newConnections.clone());
        setImportCount(importCount.get() + 1);
        items.set(newItems);
        connections.set(newConnections);
    };

    view! {
        <div style="z-index:10; position: absolute; right: 0vw; width: 20vw; height: 100%; top: 0; color : black; background-color: white">
            <Show when=move || { show.get() }>
                <div style="position: absolute; right: 2vw; width: 15vw; height: 50%; top: 0">
                    <h2>Mermaid Diagram</h2>

                    <textarea
                        style="width: 100%; height: 100%; border: 1px solid black;"
                        type="text"
                        value=text
                        on:change=move |e| setText(event_target_value(&e))
                        ref=text_area_ref
                    >
                        {text}
                    </textarea>
                    <TioButton
                        on_click=move || { handleImport() }
                        text=Signal::derive(move || "Import Diagram".to_string())
                        style="".to_string()
                    />
                </div>
            </Show>
        </div>
    }
}
