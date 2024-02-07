use crate::app::components::styling::DRAGGABLEBOX;
use leptos::html::Div;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};

#[component]
pub fn MoveBox<F: Fn() -> () + 'static>(
    id: String,
    value: String,
    position: RwSignal<Position>,
    isConnecting: ReadSignal<bool>,
    onClick: F,
) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let (editable, setEditable) = create_signal(false);
    let (name, setName) = create_signal(String::from(value.to_string()));

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
        el,
        UseDraggableOptions::default()
            .initial_value(position)
            .on_start(move |event| startDrag(event)),
    );

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
            id=id.to_string()
            style=move || {
                format!(
                    "position: fixed; {} ;border:1px solid black;background-color: #EBEBEB;width: 100px;
                        height: 100px;",
                    positionStyle.get(),
                )
            }

            on:click=move |_| { onClick() }
        >

            <div style=DRAGGABLEBOX node_ref=el>
                <div>{move || format!("{} 🤏", name.get())}</div>
                <div>
                    <Show when=move || editable.get() fallback=|| ()>
                        <p style="width: 80%; margin: 0">"Edit Name"</p>
                        <input
                            style="width: 80%; margin: 0"
                            type="text"
                            prop:value=name.get()
                            on:change=move |e| setName(event_target_value(&e))
                        />
                        <button
                            class="button small"
                            title="Close"
                            on:click=move |_| setEditable.set(false)
                        >
                            "𝖷"
                        </button>
                    </Show>
                    <Show when=move || !editable.get() fallback=|| ()>
                        <p>{name.get()}</p>
                        <button on:click=toggleEditable>Edit</button>
                    </Show>
                </div>
            </div>

        </div>
    }
}
