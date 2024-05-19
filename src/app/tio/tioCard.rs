use leptos::callback;
use leptos::html::Div;
use leptos::Children;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_element_size, UseElementSizeReturn};
use log::debug;

#[component]
pub fn TioCard(children: Children, resize: bool, size: RwSignal<Position>) -> impl IntoView {
    let cardEl = create_node_ref::<Div>();
    let scale = use_context::<RwSignal<f64>>().expect("there to be a `scale` signal provided");

    let sizeX = Signal::derive(move || size.get().x * scale.get());
    let sizeY = Signal::derive(move || size.get().y * scale.get());
    let mounted = create_rw_signal(false);
    let isResizing = create_rw_signal(false);
    let UseElementSizeReturn { width, height } = use_element_size(cardEl);

    let handleResize = move || {
        if (isResizing.get() == false) {
            return;
        }
        let sizePos = Position {
            x: width.get() / scale.get(),
            y: height.get() / scale.get(),
        };
        size.set(sizePos);
    };

    let resize_string = if resize {
        "resize: both; overflow: hidden;"
    } else {
        ""
    };

    view! {
        <div
            node_ref=cardEl
            style=move || {
                format!(
                    "background-color: #F5F5F5; margin: 2px; border-radius: 4px; border:2px solid black;{}background-color: #EEF7F4; width: {}px; height: {}px;",
                    resize_string,
                    sizeX.get(),
                    sizeY.get(),
                )
            }

            on:mousedown=move |event| {
                isResizing.set(true);
            }

            on:mouseup=move |event| { handleResize() }
        >

            {children()}
        </div>
    }
}
