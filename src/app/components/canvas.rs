use crate::app::components::connection::Connection;
use crate::app::components::diagramTextBox::DiagramTextBox;
use crate::app::components::move_box::MoveBox;
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use crate::app::tio::tioButton::TioButton;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_mouse, UseMouseReturn};
use log::debug;

#[component]
pub fn Canvas(width: i32, height: i32) -> impl IntoView {
    let (moveBoxes, setMoveBoxes) = create_signal(Vec::<RwSignal<MoveBoxItem>>::new());
    let (isConnecting, setIsConnecting) = create_signal(false);
    let (connections, setConnections) = create_signal(Vec::<ConnectionItem>::new());
    let (connectionFrom) = create_rw_signal(None::<RwSignal<MoveBoxItem>>);

    let nextPosition = create_rw_signal(Position { x: 20.0, y: 20.0 });

    let UseMouseReturn {
        x, y, source_type, ..
    } = use_mouse();

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
            attributes: create_rw_signal(vec![]),
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
                attributes: create_rw_signal(vec![]),
                key: "div0".to_string(),
                value: create_rw_signal(String::from("0")),
                position: create_rw_signal(Position { x: 20.0, y: 20.0 }),
            })
        },
        create_rw_signal(MoveBoxItem {
            attributes: create_rw_signal(vec![]),
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
                setIsConnecting(false);
            }
        }
    };

    view! {
        <div style=format!(
            "width: {}%; height: {}%; margin:0 auto;position: absolute; border-radius: 10px; border:2px solid black",
            width,
            height,
        )>
            <div style="margin: 0; position: absolute; top: 20px;  right: 5%">
                <TioButton
                    style="".to_string()
                    onClick=move || {
                        let position = nextPosition.get();
                        AddDiv(moveBoxes.get(), &setMoveBoxes, nextPosition.get());
                        nextPosition
                            .set(Position {
                                x: position.x.clone() + 50.0,
                                y: position.y.clone() + 50.0,
                            });
                    }

                    text=Signal::derive(move || {
                        format!("➕ {}", moveBoxes.get().len().to_string())
                    })
                />

                <TioButton
                    onClick=move || {
                        setIsConnecting(!isConnecting.get());
                    }

                    style="".to_string()

                    text=Signal::derive(move || {
                        if (isConnecting.get() == true) {
                            "↗️".to_string()
                        } else {
                            "🤚".to_string()
                        }
                    })
                />

            </div>
            <For each=moveBoxes key=|state| state.get().key.clone() let:child>
                <MoveBox
                    id=child.get().key
                    name=child.get().value
                    position=child.get().position
                    isConnecting=isConnecting
                    onClick=move || { connect(child) }
                    attributes=child.get().attributes
                />
            </For>
            <svg style="top: 0; left: 0; width: 100%; height: 100%;">

                <Show when=move || connectionFrom.get().is_some() fallback=|| ()>
                    <line
                        position="absolute"
                        id="temp"
                        x1=connectionFrom.get().unwrap().get().position.get().x
                        y1=connectionFrom.get().unwrap().get().position.get().y
                        x2=x
                        y2=y
                        style="stroke:rgb(0,0,0);stroke-width:2"
                    ></line>
                </Show>
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
    }
}
