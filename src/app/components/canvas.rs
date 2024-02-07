use crate::app::components::button::Button;
use crate::app::components::connection::Connection;
use crate::app::components::diagramTextBox::DiagramTextBox;
use crate::app::components::move_box::MoveBox;
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use leptos::*;
use leptos_use::core::Position;
use log::debug;

#[component]
pub fn Canvas(width: i32, height: i32) -> impl IntoView {
    let (moveBoxes, setMoveBoxes) = create_signal(Vec::<RwSignal<MoveBoxItem>>::new());
    let (isConnecting, setIsConnecting) = create_signal(false);
    let (connections, setConnections) = create_signal(Vec::<ConnectionItem>::new());
    let (connectionFrom) = create_rw_signal(None::<RwSignal<MoveBoxItem>>);

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
            value: create_rw_signal(String::from(divIds.len().to_string())),
            position: create_rw_signal(nextPosition),
        });
        divIds.push(Data);
        inSetDivIds(divIds);
    }

    setMoveBoxes(vec![
        {
            create_rw_signal(MoveBoxItem {
                key: "div0".to_string(),
                value: create_rw_signal(String::from("0")),
                position: create_rw_signal(Position { x: 20.0, y: 20.0 }),
            })
        },
        create_rw_signal(MoveBoxItem {
            key: "div1".to_string(),
            value: create_rw_signal(String::from("1")),
            position: create_rw_signal(Position { x: 200.0, y: 200.0 }),
        }),
    ]);

    let boxes = moveBoxes.get();

    setConnections(vec![ConnectionItem {
        key: "t".to_string(),
        from: boxes[0],
        to: boxes[1],
    }]);

    let connect = move |moveBoxItem: RwSignal<MoveBoxItem>| {
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
        <div style=format!(
            "width: {}%; height: {}%; margin:0 auto;position: absolute; border:1px solid black",
            width,
            height,
        )>
            <For each=moveBoxes key=|state| state.get().key.clone() let:child>
                <MoveBox
                    id=child.get().key
                    name=child.get().value
                    position=child.get().position
                    isConnecting=isConnecting
                    onClick=move || { connect(child) }
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
        <div style="display: inline-block;">
            <DiagramTextBox connections=connections items=moveBoxes/>
        </div>
        <div style="margin: 0; position: absolute; bottom: 50px;  right: 45%">
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
        </div>
    }
}
