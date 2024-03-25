use crate::app::components::attributesEditor::AttributesEditor;
use crate::app::components::nameEditor::NameEditor;
use crate::app::components::styling::NAMEBOX;
use crate::app::structs::MoveBoxAttribute::MoveBoxAttribute;
use crate::app::tio::tioCard::TioCard;
use leptos::html::{Div, P};
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{
    on_click_outside, use_draggable_with_options, UseDraggableOptions, UseDraggableReturn,
};
use log::debug;

#[component]
pub fn MoveBox<F: Fn() -> () + 'static>(
    id: String,
    name: RwSignal<String>,
    attributes: RwSignal<Vec<MoveBoxAttribute>>,
    position: RwSignal<Position>,
    realPosition: RwSignal<Position>,
    isConnecting: ReadSignal<bool>,
    isDragging: RwSignal<bool>,
    size: RwSignal<Position>,
    onClick: F,
) -> impl IntoView {
    let dragEl = create_node_ref::<Div>();
    let boxEl = create_node_ref::<Div>();
    let editable = create_rw_signal(false);

    let startDrag = move |e| {
        if !(isConnecting.get() || editable.get()) {
            isDragging.set(true);
            return true;
        }
        return false;
    };
    let UseDraggableReturn {
        x,
        y,
        style: positionStyle,
        ..
    } = use_draggable_with_options(
        dragEl,
        UseDraggableOptions::default()
            .initial_value(realPosition.get())
            .on_start(move |event| startDrag(event)),
    );

    let stopEditing = on_click_outside(boxEl, move |event| {
        editable.set(false);
    });

    let updateReal = create_effect(move |_| {
        let currentPosition = Position {
            x: x.get(),
            y: y.get(),
        };
        let realx = 69.0;
        let realy = 69.0;
        if currentPosition.x != realx || currentPosition.y != realy {
            realPosition.set(currentPosition);
        }
        //realPosition.set(currentPosition);
    });

    create_effect(move |_| {
        if isConnecting.get() {
            editable.set(false);
        }
    });

    let realX = move || realPosition.get().x;
    let realY = move || realPosition.get().y;

    let positionX = move || position.get().x;
    let positionY = move || position.get().y;

    view! {
        <div
            node_ref=boxEl
            id=id.to_string()
            style=move || {
                format!("position: fixed; {}; width: 100px; height: 200px;", positionStyle.get())
            }

            on:click=move |_| { onClick() }
        >
            <TioCard resize=true size=size>
                <div style=NAMEBOX node_ref=dragEl>
                    <NameEditor name=name editable=editable/>
                </div>
                <div>
                    <div>x: {x} , y: {y}</div>
                    <div>realx: {realX} , realy: {realY}</div>
                    <div>position: {positionX} , {positionY}</div>

                    <AttributesEditor id=id.to_string() name=name attributes=attributes/>
                </div>

            </TioCard>
        </div>
    }
}
