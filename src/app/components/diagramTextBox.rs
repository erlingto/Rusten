use leptos::*;
use leptos_use::core::{Position, Size};
use log::debug;
use styled::tracing::field::debug;

use crate::app::{
    structs::{
        connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem,
        MoveBoxAttribute::MoveBoxAttribute,
    },
    tio::tioButton::TioButton,
};

#[component]
pub fn DiagramTextBox(
    connections: ReadSignal<Vec<RwSignal<ConnectionItem>>>,
    setConnections: WriteSignal<Vec<RwSignal<ConnectionItem>>>,
    items: ReadSignal<Vec<RwSignal<MoveBoxItem>>>,
    setItems: WriteSignal<Vec<RwSignal<MoveBoxItem>>>,
) -> impl IntoView {
    let (text, setText) = create_signal(String::from(""));

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

    let importDiagram = move || {
        let diagram = text.get();
        debug!("{:?}", diagram);
        let mut newItems = vec![];
        let mut newConnections = vec![];
        let mut lines = diagram.lines();
        let mut line = lines.next();
        let mut classCount = 0;
        let mut connectionsCount = 0;

        while line.is_some() {
            let l = line.unwrap();
            if l.contains("classDiagram") {
                line = lines.next();
                continue;
            }

            if l.contains("class") {
                let mut att = vec![];
                let name = String::from(l.split("`").collect::<Vec<&str>>()[1]);
                let mut attLine = lines.next();
                let mut keyCount = 0;
                debug!("{}", l.to_string());
                while attLine.is_some() {
                    let attL = attLine.unwrap();
                    if attL.contains("}") {
                        break;
                    }
                    if attL.contains("+") {
                        att.push(MoveBoxAttribute {
                            value: create_rw_signal(attL.replace("+", "").trim().to_string()),
                            key: keyCount.to_string(),
                        });
                        keyCount += 1;
                        //att.push(attL.replace("+", "").trim().to_string());
                    }
                    attLine = lines.next();
                }
                newItems.push(create_rw_signal(MoveBoxItem {
                    position: create_rw_signal(Position { x: 0.0, y: 0.0 }),
                    value: create_rw_signal(name),
                    key: classCount.to_string(),
                    attributes: create_rw_signal(att),
                    size: create_rw_signal(Position { x: 20.0, y: 20.0 }),
                }));
                classCount += 1;
            } else if l.contains("-->") {
                let mut from = String::from("");
                let mut to = String::from("");

                let mut split = l.split("-->");
                debug!("{:?}", split);
                from = split.next().unwrap().trim().to_string();
                to = split.next().unwrap().trim().to_string();

                from = String::from(from.split("`").collect::<Vec<&str>>()[1]);
                to = String::from(to.split("`").collect::<Vec<&str>>()[1]);
                debug!("from: {}, to: {}", from, to);

                let toItem = newItems.iter().find(|x| x.get().value.get() == to);
                let fromItem = newItems.iter().find(|x| x.get().value.get() == from);
                debug!("{:?}", toItem);
                if (toItem.is_some() || fromItem.is_some()) {
                    newConnections.push(create_rw_signal(ConnectionItem {
                        key: connectionsCount.to_string(),
                        from: *fromItem.unwrap(),
                        to: *toItem.unwrap(),
                    }));
                    connectionsCount = connectionsCount + 1;
                }
            }
            line = lines.next();
        }
        debug!("{:?}", newConnections.len());
        debug!("{:?}", newItems.len());
        setItems(newItems);
        setConnections(newConnections);
    };

    view! {
        <div style="position: fixed; right: 2vw; width: 16vw; height: 50%;">
            <h2>{"Mermaid Diagram"}</h2>
            <textarea
                style="width: 100%; height: 100%; border: 1px solid black;"
                type="text"
                value=text
                on:input=move |e| setText(event_target_value(&e))
            >
                {text}
            </textarea>
            <TioButton
                onClick=move || { importDiagram() }
                text=Signal::derive(move || "Save Diagram".to_string())
                style="".to_string()
            />
        </div>
    }
}
