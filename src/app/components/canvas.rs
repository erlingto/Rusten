use crate::app::components::button::Button;
use crate::app::components::connection::Connection;
use crate::app::components::move_box::MoveBox;
use crate::app::components::structs::{ConnectionItem, MoveBoxItem};
use leptos::ev::close;
use leptos::html::S;
use leptos::*;
use leptos_use::core::Position;
use log::debug;
use web_sys::MouseEvent;

#[component]
pub fn Canvas() -> impl IntoView {
    let (moveBoxes, setDivIds) = create_signal(Vec::<MoveBoxItem>::new());
    let (isConnecting, setIsConnecting) = create_signal(false);
    let (connections, setConnections) = create_signal(Vec::<ConnectionItem>::new());
    let (connectionFrom) = create_rw_signal(None::<String>);
    let (connectionTo) = create_rw_signal(None::<String>);
    let nextPosition = create_rw_signal(Position { x: 20.0, y: 20.0 });

    fn AddDiv(
        moveBoxes: &ReadSignal<Vec<MoveBoxItem>>,
        inSetDivIds: &WriteSignal<Vec<MoveBoxItem>>,
        nextPosition: Position,
    ) {
        let ownedString = "div".to_owned();
        let mut divIds = moveBoxes.get().clone();
        let borrowedString = &divIds.len().to_string().to_owned();
        let newString = ownedString + borrowedString;
        let Data = MoveBoxItem {
            key: newString.clone(),
            value: String::from(divIds.len().to_string()),
            position: create_rw_signal(nextPosition),
        };
        divIds.push(Data);
        inSetDivIds.clone()(divIds);
    }

    setDivIds(vec![
        MoveBoxItem {
            key: "div0".to_string(),
            value: "0".to_string(),
            position: create_rw_signal(Position { x: 20.0, y: 20.0 }),
        },
        MoveBoxItem {
            key: "div1".to_string(),
            value: "1".to_string(),
            position: create_rw_signal(Position { x: 200.0, y: 200.0 }),
        },
    ]);

    setConnections(vec![ConnectionItem {
        key: "t".to_string(),
        from: moveBoxes.get()[0],
        to: moveBoxes.get()[1],
    }]);

    fn GetClosesMoveBox(moveBoxes: &Vec<MoveBoxItem>, event: MouseEvent) -> Option<String> {
        let x = event.client_x() as f64;
        let y = event.client_y() as f64;
        let mut closest: Option<String> = None;
        let mut closestDistance: f64 = 1000000.0;
        for item in moveBoxes.iter() {
            let position = item.position.get();
            let distance = ((position.x - x).powi(2) + (position.y - y).powi(2)).sqrt();
            if distance < closestDistance {
                closest = Some(item.key.clone());
                closestDistance = distance;
            }
        }
        return closest;
    }

    create_effect(move |_| {
        let from = connectionFrom.get();
        let to = connectionTo.get();

        if from.is_some() && to.is_some() {
            let mut newConnections = connections.get().clone();
            let fromCondition = |&item: &&MoveBoxItem| item.key == from.unwrap();
            let toCondition = |&item: &&MoveBoxItem| item.key == to.unwrap();
            let Some(fromItem) = moveBoxes.get().iter().find(fromCondition) else {
                return;
            };
            let Some(toItem) = moveBoxes.get().iter().find(toCondition) else {
                return;
            };

            let newConnection = ConnectionItem {
                key: newConnections.len().to_string(),
                from: fromItem.clone(),
                to: toItem.clone(),
            };
            newConnections.push(newConnection);
            setConnections(newConnections);
            connectionFrom.set(None);
            connectionTo.set(None);
        }
    });

    fn Connect(
        event: MouseEvent,
        isConnecting: &ReadSignal<bool>,
        moveBoxes: &ReadSignal<Vec<MoveBoxItem>>,
        connectionFrom: &RwSignal<Option<String>>,
        connectionTo: &RwSignal<Option<String>>,
    ) {
        let movetemp = &moveBoxes.get();
        if isConnecting.get() {
            debug!("Event: {:?}", event);
            let closest = GetClosesMoveBox(movetemp, event);
            if closest.is_some() {
                if connectionFrom.get().is_none() {
                    connectionFrom.set(closest);
                } else {
                    connectionTo.set(closest);
                }
            }
        }

        debug!("Connecting: {}", isConnecting.get());
    }

    view! {
        <div style="width: 100%; height: 100%; margin:0 auto;position: absolute">
            <For each=moveBoxes key=|state| state.key.clone() let:child>
                <MoveBox
                    id=child.key.clone()
                    value=child.value.clone()
                    position=child.position
                    isConnecting=isConnecting
                />
            </For>
            <svg
                style="top: 0; left: 0; width: 100%; height: 100%;"
                on:click=move |event| {
                    Connect(event, &isConnecting, &moveBoxes, &connectionFrom, &connectionTo);
                }
            >

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
                    AddDiv(&moveBoxes, &setDivIds, nextPosition.get());
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
