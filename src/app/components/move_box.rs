use crate::app::components::attributesEditor::AttributesEditor;
use crate::app::components::nameEditor::NameEditor;
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
#[component]
pub fn MoveBox<F: Fn() -> () + 'static>(
    is_connecting: RwSignal<bool>,
    move_box_item: RwSignal<MoveBoxItem>,
    onClick: F,
) -> impl IntoView {
    let dragEl = create_node_ref::<Div>();
    let boxEl = create_node_ref::<Div>();
    let editable = create_rw_signal(false);

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
            x: previousRealPosition.x + x - previousPosition.x,
            y: previousRealPosition.y + y - previousPosition.y,
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
