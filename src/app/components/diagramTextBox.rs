use crate::app::dictionary::dict;
use leptos::*;
use leptos_use::core::{Position, Size};
use log::debug;

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
    let (importCount, setImportCount) = create_signal(0);

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

    let organizePositions = move |connections: Vec<RwSignal<ConnectionItem>>,
                                  items: Vec<RwSignal<MoveBoxItem>>| {
        let mut toRankDict = dict::<String, i32>::new();
        let mut fromRankDict = dict::<String, i32>::new();

        items.iter().for_each(|x| {
            let key = x.get().key.to_string();
            toRankDict.insert(key.clone(), 0);
            fromRankDict.insert(key.clone(), 0);
        });

        connections.iter().for_each(|x| {
            let from = x.get().from.get().key.to_string();
            let to = x.get().to.get().key.to_string();
            toRankDict.insert(to.clone(), toRankDict.get(to).unwrap() + 1);
            fromRankDict.insert(from.clone(), fromRankDict.get(from.clone()).unwrap() + 1);
        });

        let mut toRankneighbourWeighted = dict::<String, i32>::new();
        let mut fromRankneighbourWeighted = dict::<String, i32>::new();

        items.iter().for_each(|x| {
            let key = x.get().key.to_string();
            toRankneighbourWeighted
                .insert(key.clone(), toRankDict.get(key.clone()).unwrap().clone());
            fromRankneighbourWeighted
                .insert(key.clone(), fromRankDict.get(key.clone()).unwrap().clone());
        });

        connections.iter().for_each(|x| {
            let from = x.get().from.get().key.to_string();
            let to = x.get().to.get().key.to_string();
            let toNeighbourValue = toRankDict.get(from.clone());
            let fromNeighbourValue = fromRankDict.get(to.clone());
            if (toNeighbourValue.is_some()) {
                toRankneighbourWeighted.insert(
                    to.clone(),
                    *toNeighbourValue.unwrap() + toRankneighbourWeighted.get(to.clone()).unwrap(),
                );
            }
            if (fromNeighbourValue.is_some()) {
                fromRankneighbourWeighted.insert(
                    from.clone(),
                    *fromNeighbourValue.unwrap()
                        + fromRankneighbourWeighted.get(from.clone()).unwrap(),
                );
            }
        });

        let mut toRankOverView = dict::<i32, Vec<String>>::new();
        let mut fromRankOverView = dict::<i32, Vec<String>>::new();

        toRankDict.into_iter().for_each(|x| {
            if toRankOverView.get(x.1).is_none() {
                toRankOverView.insert(x.1, vec![x.0]);
            } else {
                toRankOverView[x.1].push(x.0);
            }
        });

        fromRankDict.into_iter().for_each(|x| {
            if fromRankOverView.get(x.1).is_none() {
                fromRankOverView.insert(x.1, vec![x.0]);
            } else {
                fromRankOverView[x.1].push(x.0);
            }
        });

        let Ylevels = toRankOverView.clone().into_iter().len() as i32;
        let mut Xlevels = vec![];
        toRankOverView.clone().into_iter().for_each(|y| {
            Xlevels.push(y.1.len() as i32);
        });

        items.iter().for_each(|item| {
            debug!("{:?}", item.get().key);
            debug!("{:?}", toRankneighbourWeighted.clone());
            let rank = toRankneighbourWeighted
                .clone()
                .into_iter()
                .find(|x| x.0 == item.get().key)
                .unwrap()
                .1;
            let xRankList = toRankOverView
                .clone()
                .into_iter()
                .find(|x| x.1.contains(&item.get().key))
                .unwrap()
                .1;
            let xRank = xRankList.iter().position(|x| x == &item.get().key).unwrap() as i32;
            let size = item.get().size.get();
            debug!("{:?}", size.y);
            let yPosition = (100 + (size.y + 145.0) as i32 * rank) as f64;
            let xPosition = (500 + (size.x + 135.0) as i32 * xRank) as f64;
            item.get().position.set(Position {
                x: xPosition,
                y: yPosition,
            });
        })
    };

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
                    }
                    attLine = lines.next();
                }
                newItems.push(create_rw_signal(MoveBoxItem {
                    position: create_rw_signal(Position { x: 0.0, y: 0.0 }),
                    value: create_rw_signal(name),
                    key: format!(
                        "{}:{}",
                        importCount.get().to_string(),
                        classCount.to_string()
                    ),
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
                if (toItem.is_some() && fromItem.is_some()) {
                    newConnections.push(create_rw_signal(ConnectionItem {
                        key: format!(
                            "{}_{}",
                            importCount.get().to_string(),
                            connectionsCount.to_string()
                        ),
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
        organizePositions(newConnections.clone(), newItems.clone());
        setImportCount(importCount.get() + 1);
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
                text=Signal::derive(move || "Import Diagram".to_string())
                style="".to_string()
            />
        </div>
    }
}
