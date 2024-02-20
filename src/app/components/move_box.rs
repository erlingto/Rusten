use crate::app::components::attributesEditor::AttributesEditor;
use crate::app::components::nameEditor::NameEditor;
use crate::app::components::styling::{NAMEBOX, TEXTINPUT};
use crate::app::structs::MoveBoxAttribute::MoveBoxAttribute;
use crate::app::tio::tioCard::TioCard;
use leptos::html::{Div, P};
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{
    on_click_outside, use_draggable_with_options, use_element_size, UseDraggableOptions,
    UseDraggableReturn, UseElementSizeReturn,
};
use log::debug;

#[component]
pub fn MoveBox<F: Fn() -> () + 'static>(
    id: String,
    name: RwSignal<String>,
    attributes: RwSignal<Vec<MoveBoxAttribute>>,
    position: RwSignal<Position>,
    isConnecting: ReadSignal<bool>,
    size: RwSignal<Position>,
    onClick: F,
) -> impl IntoView {
    let dragEl = create_node_ref::<Div>();
    let boxEl = create_node_ref::<Div>();
    let editable = create_rw_signal(false);

    let startDrag = move |e| !(isConnecting.get() || editable.get());
    let UseDraggableReturn {
        x,
        y,
        style: positionStyle,
        ..
    } = use_draggable_with_options(
        dragEl,
        UseDraggableOptions::default()
            .initial_value(position)
            .on_start(move |event| startDrag(event)),
    );

    let stopEditing = on_click_outside(boxEl, move |event| {
        editable.set(false);
    });

    create_effect(move |_| {
        let currentPosition = Position {
            x: x.get(),
            y: y.get(),
        };
        let previousPosition = position.get();
        if (currentPosition.x != previousPosition.x || currentPosition.y != previousPosition.y) {
            position.set(currentPosition);
        }
    });

    create_effect(move |_| {
        if isConnecting.get() {
            editable.set(false);
        }
    });

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
                    <AttributesEditor id=id.to_string() name=name attributes=attributes/>
                </div>

            </TioCard>
        </div>
    }
}
