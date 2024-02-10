use crate::app::components::attributeEditor::AttributeEditor;
use crate::app::components::styling::DRAGGABLEBOX;
use crate::app::tio::tioCard::TioCard;
use leptos::html::Div;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{
    on_click_outside, use_draggable_with_options, UseDraggableOptions, UseDraggableReturn,
};

#[component]
pub fn MoveBox<F: Fn() -> () + 'static>(
    id: String,
    name: RwSignal<String>,
    attributes: RwSignal<Vec<RwSignal<String>>>,
    position: RwSignal<Position>,
    isConnecting: ReadSignal<bool>,
    onClick: F,
) -> impl IntoView {
    let dragEl = create_node_ref::<Div>();
    let boxEl = create_node_ref::<Div>();
    let (editable, setEditable) = create_signal(false);

    let startDrag = move |e| !(isConnecting.get() || editable.get());
    let toggleEditable = move |_| {
        setEditable(true);
    };
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
            <TioCard>
                <div style=DRAGGABLEBOX node_ref=dragEl>
                    <div>{move || format!("{} 🤏", name.get())}</div>
                </div>
                <div>
                    <Show when=move || editable.get() fallback=|| ()>
                        <AttributeEditor id=id.to_string() name=name attributes=attributes/>
                    </Show>
                    <Show when=move || !editable.get() fallback=|| ()>
                        <For each=attributes key=|state| state.get() let:child>
                            <p>{child.get()}</p>
                        </For>
                        <button on:click=toggleEditable>Edit</button>
                    </Show>
                </div>

            </TioCard>
        </div>
    }
}
