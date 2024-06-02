use leptos::html::Div;
use leptos::Children;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_element_size, UseElementSizeReturn};

#[component]
pub fn TioComponent(children: Children) -> impl IntoView {
    let size = create_rw_signal(Position { x: 100.0, y: 100.0 });
    let count = create_rw_signal(0);
    let effect = create_effect(move || {
        let countNow = count.get();
        size.set(Position {
            x: 10.0 * countNow,
            y: 10.0 * countNow,
        })
    });

    let styleClosure = move || {
        format!(
            "background-color: #F5F5F5; margin: 2px; border-radius: 4px; border:2px solid black; resize; background-color: #EEF7F4; width: {}px; height: {}px;",
            size.get().x,
            size.get().y,
        )
    };

    view! {
        <button
            style=styleClosure
            on:mousedown=move |event| {
                count.set(count.get() + 1);
            }
        >

            {children()}
        </button>
    }
}
