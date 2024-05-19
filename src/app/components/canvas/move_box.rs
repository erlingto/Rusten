use crate::app::components::canvas::attributesEditor::AttributesEditor;
use crate::app::components::canvas::nameEditor::NameEditor;
use crate::app::components::styling::NAMEBOX;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use crate::app::tio::tioCard::TioCard;
use leptos::html::Div;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{
    on_click_outside, use_draggable_with_options, UseDraggableCallbackArgs, UseDraggableOptions,
    UseDraggableReturn,
};
use log::debug;
#[component]
pub fn MoveBox<F: Fn() -> () + 'static>(
    is_connecting: RwSignal<bool>,
    scale: RwSignal<f64>,
    move_box_item: RwSignal<MoveBoxItem>,
    on_click: F,
) -> impl IntoView {
    let dragEl = create_node_ref::<Div>();
    let boxEl = create_node_ref::<Div>();
    let editable = create_rw_signal(false);
    let should_render = move_box_item.get().should_render;

    let isDragging = move_box_item.get().isDragging;
    let position = move_box_item.get().position;
    let realPosition = move_box_item.get().realPosition;
    let name = move_box_item.get().value;
    let attributes = move_box_item.get().attributes;
    let size = move_box_item.get().size;
    let id = move_box_item.get().key;

    let position_x_in_div = create_rw_signal(0.0);
    let position_y_in_div = create_rw_signal(0.0);

    let startDrag = move |e: UseDraggableCallbackArgs| {
        if !(is_connecting.get() || editable.get()) {
            isDragging.set(true);
            e.event.prevent_default();

            if let Some(target) = dragEl.get_untracked() {
                let rect = target.get_bounding_client_rect();
                let x = e.event.x() as f64 - rect.left();
                let y = e.event.y() as f64 - rect.top();
                position_x_in_div.set(x);
                position_y_in_div.set(y);
            }

            return true;
        }
        return false;
    };

    let updateRealPosition = move |event: UseDraggableCallbackArgs| {
        isDragging.set(false);
        let previousPosition = position.get();
        let previousRealPosition = realPosition.get();

        let x = event.event.x() as f64 - position_x_in_div.get();
        let y = event.event.y() as f64 - position_y_in_div.get();
        let newRealPosition = Position {
            x: previousRealPosition.x + (x - previousPosition.x) / scale.get(),
            y: previousRealPosition.y + (y - previousPosition.y) / scale.get(),
        };
        realPosition.set(newRealPosition)
    };

    let UseDraggableReturn {
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

    let _ = on_click_outside(boxEl, move |_| {
        editable.set(false);
    });

    create_effect(move |_| {
        if is_connecting.get() {
            editable.set(false);
        }
    });

    view! {
        <div
            display=move || { if should_render.get() { "inline-flex;" } else { "none;" } }

            node_ref=boxEl
            id=id.to_string()
            style=move || {
                format!(
                    "z-index: 1; position: fixed; {}; display: {};",
                    positionStyle.get(),
                    if should_render.get() { "block" } else { "none" },
                )
            }

            on:click=move |_| { on_click() }
        >
            <TioCard resize=true size=size>
                <div style=NAMEBOX node_ref=dragEl>
                    <NameEditor name=name editable=editable/>
                </div>

                <div style=move || { format!("height: {}px;", 22.0 * scale.get()) }>
                    <AttributesEditor id=id.to_string() attributes=attributes/>
                </div>
            </TioCard>
        </div>
    }
}
