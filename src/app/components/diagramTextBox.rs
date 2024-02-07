use leptos::*;
use log::debug;

use crate::app::structs::{connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem};

#[component]
pub fn DiagramTextBox(
    connections: ReadSignal<Vec<ConnectionItem>>,
    items: ReadSignal<Vec<RwSignal<MoveBoxItem>>>,
) -> impl IntoView {
    let (text, setText) = create_signal(String::from(""));

    let printDiagram = move || {
        let mut connectionString = String::from(":::mermaid\n");
        connectionString.push_str("classDiagram\n");
        items.get().iter().for_each(|x| {
            connectionString.push_str(&format!("class `{}`\n", x.get().value.get()));
        });

        connections.get().iter().for_each(|x| {
            connectionString.push_str(&format!(
                "`{}` --> `{}`\n",
                x.clone().from.get().value.get(),
                x.clone().to.get().value.get()
            ));
        });
        setText(connectionString);
    };
    printDiagram();
    create_effect(move |_| {
        printDiagram();
    });

    view! {
        <div style="position: fixed; right: 2vw; width: 16vw; height: 50%;">
            <h2>{"Mermaid Diagram"}</h2>
            <div style="width: 100%; height: 100%; border: 1px solid black;">
                <textarea
                    style="width: 100%; height: 100%;"
                    type="text"
                    value=text
                    on:input=move |e| setText(event_target_value(&e))
                >
                    {text}
                </textarea>
                <button on:click=move |_| { printDiagram() }>{"Save Diagram"}</button>
            </div>
        </div>
    }
}
