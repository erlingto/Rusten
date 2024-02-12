use crate::app::components::attributesEditor::AttributesEditor;
use crate::app::components::styling::DRAGGABLEBOX;
use crate::app::structs::MoveBoxAttribute::MoveBoxAttribute;
use crate::app::tio::tioCard::TioCard;
use leptos::html::Div;
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
    isConnecting: ReadSignal<bool>,
    onClick: F,
) -> impl IntoView {
    let dragEl = create_node_ref::<Div>();
    let boxEl = create_node_ref::<Div>();
    let (editable, setEditable) = create_signal(false);

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
        setEditable(false);
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
            setEditable(false);
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
            <TioCard resize=true>
                <div style=DRAGGABLEBOX node_ref=dragEl>
                    <div>{move || format!("{} 🤏", name.get())}</div>
                </div>
                <div>
                    <AttributesEditor id=id.to_string() name=name attributes=attributes/>
                </div>

            </TioCard>
        </div>
    }
}
