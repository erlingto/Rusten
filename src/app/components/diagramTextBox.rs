use crate::app::helpers::{orderFunctions::organize_positions, parseFunctions::importDiagram};
use leptos::*;
use leptos_router::use_query_map;
use log::debug;

use crate::app::{
    structs::{connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem},
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
    let (disableImport, setDisableImport) = create_signal(true);

    let urlState = use_query_map();

    let handleImport = move || {
        let (mut newItems, mut newConnections) = importDiagram(text.get(), importCount.get());
        newItems = organize_positions(newItems, newConnections.clone());
        setImportCount(importCount.get() + 1);
        items.set(newItems);
        connections.set(newConnections);
    };

    let _ = create_effect(move |_| {
        let urlDiagramString = urlState.with(|params| params.get("diagram").cloned());

        if (urlDiagramString.is_some()
            && importDiagram(urlDiagramString.clone().unwrap(), importCount.get())
                != (vec![], vec![]))
        {
            setText(urlDiagramString.unwrap());
            handleImport();
        };
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
        setText(newText.clone());
        text_area_ref
            .get()
            .unwrap()
            .set_value(newText.clone().as_str());
    });

    view! {
        <div style="z-index:10; position: absolute; right: 0vw; width: 20vw; height: 100%; top: 0; color : black; background-color: white">
            <div style="position: absolute; right: 2vw; width: 15vw; height: 50%; top: 0">
                <h2>Mermaid Diagram</h2>

                <textarea
                    id=move || {
                        importCount.get().to_string() + items.get().len().to_string().as_str()
                    }

                    style="width: 100%; height: 100%; border: 1px solid black;"
                    type="text"
                    value=move || text.get()
                    on:input=move |e| {
                        setDisableImport(false);
                        e.prevent_default();
                        setText(event_target_value(&e))
                    }

                    ref=text_area_ref
                ></textarea>
                <TioButton
                    disabled=Signal::derive(disableImport)
                    on_click=move || {
                        handleImport();
                        setDisableImport(true);
                    }

                    text=Signal::derive(move || "Import Diagram".to_string())
                    style="".to_string()
                />
            </div>
        </div>
    }
}
