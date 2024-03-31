use crate::app::components::canvasForever::CanvasForever;
use crate::app::components::diagramTextBox::DiagramTextBox;
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use crate::app::tio::tioButton::TioButton;
use leptos::*;
use leptos_use::core::Position;

#[component]
pub fn MermaidEditor() -> impl IntoView {
    let moveBoxes = create_rw_signal(Vec::<RwSignal<MoveBoxItem>>::new());
    let connections = create_rw_signal(Vec::<RwSignal<ConnectionItem>>::new());
    let is_connecting = create_rw_signal(false);

    let new_connection_start = create_rw_signal(None::<RwSignal<MoveBoxItem>>);
    let nextPosition = create_rw_signal(Position { x: 20.0, y: 20.0 });

    fn AddDiv(moveBoxes: RwSignal<Vec<RwSignal<MoveBoxItem>>>, nextPosition: Position) {
        let ownedString = "div".to_owned();
        let mut newMoveBoxes = moveBoxes.get();
        let borrowedString = &newMoveBoxes.len().to_string().to_owned();
        let newString = ownedString + borrowedString;
        let Data = create_rw_signal(MoveBoxItem {
            attributes: create_rw_signal(vec![]),
            key: newString.clone(),
            value: create_rw_signal(String::from(newMoveBoxes.len().to_string())),
            position: create_rw_signal(nextPosition),
            realPosition: create_rw_signal(nextPosition),
            isDragging: create_rw_signal(false),
            size: create_rw_signal(Position { x: 100.0, y: 200.0 }),
        });
        newMoveBoxes.push(Data);
        moveBoxes.set(newMoveBoxes);
    }

    moveBoxes.set(vec![
        {
            create_rw_signal(MoveBoxItem {
                attributes: create_rw_signal(vec![]),
                key: "start0".to_string(),
                value: create_rw_signal(String::from("start0")),
                position: create_rw_signal(Position { x: 20.0, y: 20.0 }),
                realPosition: create_rw_signal(Position { x: 20.0, y: 20.0 }),
                isDragging: create_rw_signal(false),
                size: create_rw_signal(Position { x: 100.0, y: 200.0 }),
            })
        },
        create_rw_signal(MoveBoxItem {
            attributes: create_rw_signal(vec![]),
            key: "start1".to_string(),
            value: create_rw_signal(String::from("start1")),
            position: create_rw_signal(Position { x: 200.0, y: 200.0 }),
            realPosition: create_rw_signal(Position { x: 200.0, y: 200.0 }),
            isDragging: create_rw_signal(false),
            size: create_rw_signal(Position { x: 100.0, y: 200.0 }),
        }),
    ]);

    let boxes = moveBoxes.get();

    connections.set(vec![create_rw_signal(ConnectionItem {
        key: "0".to_string(),
        from: boxes[0],
        to: boxes[1],
    })]);

    view! {
        <CanvasForever
            items=moveBoxes
            connections=connections
            is_connecting=is_connecting
            new_connection_start=new_connection_start
        />
        <div style="margin: 0; position: absolute; top: 40px;  right: 25vw">
            <TioButton
                style="".to_string()
                on_click=move || {
                    let position = nextPosition.get();
                    AddDiv(moveBoxes, nextPosition.get());
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
                on_click=move || {
                    is_connecting.set(!is_connecting.get());
                }

                style="".to_string()

                text=Signal::derive(move || {
                    if is_connecting.get() == true {
                        "↗️".to_string()
                    } else {
                        "🤚".to_string()
                    }
                })
            />

        </div>
        <DiagramTextBox connections=connections items=moveBoxes/>
    }
}
