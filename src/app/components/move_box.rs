use std::f64::consts::E;

use crate::app::components::attributesEditor::AttributesEditor;
use crate::app::components::nameEditor::NameEditor;
use crate::app::components::styling::NAMEBOX;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use crate::app::structs::MoveBoxAttribute::MoveBoxAttribute;
use crate::app::tio::tioCard::TioCard;
use leptos::html::{Div, P};
use leptos::math::Mo;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{
    on_click_outside, use_draggable_with_options, UseDraggableCallbackArgs, UseDraggableOptions,
    UseDraggableReturn,
};
use log::debug;
use web_sys::EventTarget;

#[component]
pub fn MoveBox<F: Fn() -> () + 'static>(
    isConnecting: ReadSignal<bool>,
    MoveBoxItem: RwSignal<MoveBoxItem>,
    onClick: F,
) -> impl IntoView {
    let dragEl = create_node_ref::<Div>();
    let boxEl = create_node_ref::<Div>();
    let editable = create_rw_signal(false);

    let isDragging = MoveBoxItem.get().isDragging;
    let position = MoveBoxItem.get().position;
    let realPosition = MoveBoxItem.get().realPosition;
    let name = MoveBoxItem.get().value;
    let attributes = MoveBoxItem.get().attributes;
    let size = MoveBoxItem.get().size;
    let id = MoveBoxItem.get().key;

    let positionXinDiv = create_rw_signal(0.0);
    let positionYinDiv = create_rw_signal(0.0);

    let startDrag = move |e: UseDraggableCallbackArgs| {
        if !(isConnecting.get() || editable.get()) {
            isDragging.set(true);

            if let Some(target) = dragEl.get_untracked() {
                let rect = target.get_bounding_client_rect();
                let x = e.event.x() as f64 - rect.left();
                let y = e.event.y() as f64 - rect.top();
                positionXinDiv.set(x);
                positionYinDiv.set(y);
            }

            return true;
        }
        return false;
    };

    let updateRealPosition = (move |event: UseDraggableCallbackArgs| {
        isDragging.set(false);
        let previousPosition = position.get();

        if let Some(target) = dragEl.get_untracked() {
            let rect = target.get_bounding_client_rect();
            let x = event.event.x() as f64 - positionXinDiv.get();
            let y = event.event.y() as f64 - positionYinDiv.get();
            let previousRealPosition = realPosition.get();
            let newRealPosition = Position {
                x: previousRealPosition.x + x - previousPosition.x,
                y: previousRealPosition.y + y - previousPosition.y,
            };
            realPosition.set(newRealPosition)
        }
    });

    let UseDraggableReturn {
        x,
        y,
        style: positionStyle,
        ..
    } = use_draggable_with_options(
        dragEl,
        UseDraggableOptions::default()
            .initial_value(position)
            .on_start(move |event| startDrag(event))
            .on_end(move |event| {
                updateRealPosition(event);
            }),
    );

    let stopEditing = on_click_outside(boxEl, move |event| {
        editable.set(false);
    });

    create_effect(move |_| {
        if isConnecting.get() {
            editable.set(false);
        }
    });

    let positionx = move || position.get().x;
    let positiony = move || position.get().y;

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
