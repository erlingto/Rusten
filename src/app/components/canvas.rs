use crate::app::components::button::Button;
use crate::app::components::structs::DivItem;
use crate::app::components::move_box::MoveBox;
use leptos::*;
use leptos_use::core::Position;



#[component]
pub fn Canvas() -> impl IntoView {
    let (divIds, setDivIds) = create_signal(Vec::<DivItem>::new());
    let (nextPosition, setNextPosition) = create_signal(Position {
        x: 0.0,
        y: 0.0,
    });

    fn AddDiv(inDivIds: &ReadSignal<Vec<DivItem>>, inSetDivIds: &WriteSignal<Vec<DivItem>>, nextPosition: Position) {
        let ownedString = "div".to_owned();
        let mut divIds = inDivIds.get().clone();
        let borrowedString = &divIds.len().to_string().to_owned();
        let newString = ownedString + borrowedString;
        let Data = DivItem {
            key: newString.clone(),
            value: divIds.len() as i32,
            position: nextPosition,
        };
        divIds.push(Data);
        inSetDivIds.clone()(divIds);
    }

    view! {
        <div style="width:800px; margin:0 auto;">
            <For
                each=divIds
                key=|state| state.key.clone()
                let:child
            >
                <MoveBox id={child.key.clone()} value={child.value} position={child.position.clone()}/>
            </For>


                
        </div>
        <div style="margin: 0; position: absolute; bottom: 200px;  right: 45%" >
            <Button onClick=move || {
                let position = nextPosition.get();
                AddDiv(&divIds, &setDivIds, nextPosition.get());
                setNextPosition(Position {
                    x: position.x.clone() + 50.0,
                    y: position.y.clone() + 50.0,
                });
            }/>
        </div>
    }
}
