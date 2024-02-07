use crate::app::components::button::Button;
use crate::app::components::connection::Connection;
use crate::app::components::move_box::MoveBox;
use crate::app::components::structs::{ConnectionItem, MoveBoxItem};
use leptos::*;
use leptos_use::core::Position;
use log::debug;

#[component]
pub fn Canvas() -> impl IntoView {
    let (moveBoxes, setMoveBoxes) = create_signal(Vec::<RwSignal<MoveBoxItem>>::new());
    let (isConnecting, setIsConnecting) = create_signal(false);
    let (connections, setConnections) = create_signal(Vec::<ConnectionItem>::new());
    let (connectionFrom) = create_rw_signal(None::<RwSignal<MoveBoxItem>>);

    let printDiagram = move || {
        let mut connectionString = String::from(":::mermaid\n");
        connectionString.push_str("classDiagram\n");
        moveBoxes.get().iter().for_each(|x| {
            connectionString.push_str(&format!("class `{}`\n", x.get().value));
        });

        connections.get().iter().for_each(|x| {
            connectionString.push_str(&format!(
                "`{}` --> `{}`",
                x.clone().from.get().value,
                x.clone().to.get().value
            ));
        });
        debug!("{}", connectionString);
    };

    let nextPosition = create_rw_signal(Position { x: 20.0, y: 20.0 });

    fn AddDiv(
        moveBoxes: Vec<RwSignal<MoveBoxItem>>,
        inSetDivIds: &WriteSignal<Vec<RwSignal<MoveBoxItem>>>,
        nextPosition: Position,
    ) {
        let ownedString = "div".to_owned();
        let mut divIds = moveBoxes.clone();
        let borrowedString = &divIds.len().to_string().to_owned();
        let newString = ownedString + borrowedString;
        let Data = create_rw_signal(MoveBoxItem {
            key: newString.clone(),
            value: String::from(divIds.len().to_string()),
            position: create_rw_signal(nextPosition),
        });
        divIds.push(Data);
        inSetDivIds(divIds);
    }

    setMoveBoxes(vec![
        {
            create_rw_signal(MoveBoxItem {
                key: "div0".to_string(),
                value: "0".to_string(),
                position: create_rw_signal(Position { x: 20.0, y: 20.0 }),
            })
        },
        create_rw_signal(MoveBoxItem {
            key: "div1".to_string(),
            value: "1".to_string(),
            position: create_rw_signal(Position { x: 200.0, y: 200.0 }),
        }),
    ]);

    let boxes = moveBoxes.get();

    setConnections(vec![ConnectionItem {
        key: "t".to_string(),
        from: boxes[0],
        to: boxes[1],
    }]);

    let connect_2 = move |moveBoxItem: RwSignal<MoveBoxItem>| {
        if isConnecting.get() {
            if (connectionFrom.get().is_none()) {
                connectionFrom.set(Some(moveBoxItem));
            } else {
                let from = connectionFrom.get().unwrap();
                let mut newConnections = connections.get();
                let newConnection = ConnectionItem {
                    key: connections.get().len().to_string(),
                    from: from,
                    to: moveBoxItem,
                };
                newConnections.push(newConnection);
                setConnections(newConnections);
                connectionFrom.set(None);
            }
        }
    };

    view! {
        <div style="width: 100%; height: 100%; margin:0 auto;position: absolute">
            <For each=moveBoxes key=|state| state.get().key.clone() let:child>
                <MoveBox
                    id=child.get().key
                    value=child.get().value
                    position=child.get().position
                    isConnecting=isConnecting
                    onClick=move || { connect_2(child) }
                />

            </For>
            <svg style="top: 0; left: 0; width: 100%; height: 100%;">

                <For each=connections key=|state| state.key.clone() let:connection>
                    <Connection
                        onClick=move || {
                            debug!("Connection: {}", "Clicked");
                        }

                        data=connection.clone()
                    />
                </For>
            </svg>
        </div>

        <div style="margin: 0; position: absolute; bottom: 200px;  right: 45%">
            <Button
                onClick=move || {
                    let position = nextPosition.get();
                    AddDiv(moveBoxes.get(), &setMoveBoxes, nextPosition.get());
                    nextPosition
                        .set(Position {
                            x: position.x.clone() + 50.0,
                            y: position.y.clone() + 50.0,
                        });
                }

                title=format!("Add MoveBox:")
                signal=Signal::derive(move || moveBoxes.get().len().to_string())
            />
            <Button
                onClick=move || {
                    setIsConnecting(!isConnecting.get());
                }

                title=format!("Connecting")
                signal=Signal::derive(move || isConnecting.get().to_string())
            />
            <Button
                onClick=move || {
                    printDiagram();
                }

                title=format!("Print")
                signal=Signal::derive(move || isConnecting.get().to_string())
            />
        </div>
    }
}
