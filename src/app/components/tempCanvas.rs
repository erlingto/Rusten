use crate::app::components::canvasForever::CanvasForever;
use crate::app::components::connection::Connection;
use crate::app::components::diagramTextBox::DiagramTextBox;
use crate::app::components::move_box::MoveBox;
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use crate::app::tio::tioButton::TioButton;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_mouse, UseMouseReturn};

#[component]
pub fn Canvas(width: i32, height: i32) -> impl IntoView {
    let (moveBoxes, setMoveBoxes) = create_signal(Vec::<RwSignal<MoveBoxItem>>::new());
    let (isConnecting, setIsConnecting) = create_signal(false);
    let (connections, setConnections) = create_signal(Vec::<RwSignal<ConnectionItem>>::new());
    let connectionFrom = create_rw_signal(None::<RwSignal<MoveBoxItem>>);

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
            size: create_rw_signal(Position { x: 100.0, y: 200.0 }),
        });
        divIds.push(Data);
        inSetDivIds(divIds);
    }

    setMoveBoxes(vec![
        {
            create_rw_signal(MoveBoxItem {
                attributes: create_rw_signal(vec![]),
                key: "start0".to_string(),
                value: create_rw_signal(String::from("start0")),
                position: create_rw_signal(Position { x: 20.0, y: 20.0 }),
                size: create_rw_signal(Position { x: 100.0, y: 200.0 }),
            })
        },
        create_rw_signal(MoveBoxItem {
            attributes: create_rw_signal(vec![]),
            key: "start1".to_string(),
            value: create_rw_signal(String::from("start1")),
            position: create_rw_signal(Position { x: 200.0, y: 200.0 }),
            size: create_rw_signal(Position { x: 100.0, y: 200.0 }),
        }),
    ]);

    let boxes = moveBoxes.get();

    setConnections(vec![create_rw_signal(ConnectionItem {
        key: "0".to_string(),
        from: boxes[0],
        to: boxes[1],
    })]);

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
                newConnections.push(create_rw_signal(newConnection));
                setConnections(newConnections);
                connectionFrom.set(None);
                setIsConnecting(false);
            }
        }
    };

    view! { <CanvasForever width=width height=height/> }
}
