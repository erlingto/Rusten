use leptos::{svg::Line, *};
use leptos_use::{core::Position, use_element_hover_with_options, UseElementHoverOptions};
use log::debug;

use crate::app::structs::{connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem};
#[derive(Debug, Clone)]
pub struct LinePosition {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

#[component]
pub fn Connection<F: Fn() -> () + 'static>(
    onClick: F,
    data: RwSignal<ConnectionItem>,
) -> impl IntoView {
    let el = create_node_ref::<Line>();
    let is_hovered = use_element_hover_with_options(el, UseElementHoverOptions::default());

    let calculateFromAndTo = move |to: MoveBoxItem, from: MoveBoxItem| {
        let fromPosition = from.position.get();
        let toPosition = to.position.get();
        let fromSize = from.size.get();
        let toSize = to.size.get();
        let fromX = fromPosition.x + fromSize.x / 2.0;
        let fromY = fromPosition.y + fromSize.y / 2.0;
        let closest_x = max(
            (toPosition.x + 2.0 - 13.0),
            min(fromX, toPosition.x + toSize.x - 2.0),
        );
        let closest_y = max(
            (toPosition.y + 2.0 - 13.0),
            min(fromY, toPosition.y + toSize.y - 2.0),
        );

        LinePosition {
            x1: fromX,
            y1: fromY,
            x2: closest_x,
            y2: closest_y,
        }
    };
    let linePos = create_rw_signal(calculateFromAndTo(
        data.get().to.get(),
        data.get().from.get(),
    ));

    create_effect(move |_| {
        let newLinePos = calculateFromAndTo(data.get().to.get(), data.get().from.get());
        if (newLinePos.x1 != linePos.get().x1
            || newLinePos.y1 != linePos.get().y1
            || newLinePos.x2 != linePos.get().x2
            || newLinePos.y2 != linePos.get().y2)
        {
            linePos.set(newLinePos);
        }
    });

    view! {
        <marker
            id=data.get().key
            viewBox="0 0 10 10"
            refX="5"
            refY="5"
            markerWidth="6"
            markerHeight="6"
            orient="auto-start-reverse"
        >
            <path
                d="M 0 0 L 10 5 L 0 10 z"
                fill=move || { if is_hovered.get() { "red" } else { "black" } }
            ></path>
        </marker>
        <line
            style="position:absolute; cursor: pointer; padding: 20px; z-index: 1;"
            padding="20px"
            node_ref=el
            x1=move || linePos.get().x1
            y1=move || linePos.get().y1
            x2=move || linePos.get().x2
            y2=move || linePos.get().y2
            stroke=move || { if is_hovered.get() { "red" } else { "black" } }
            stroke-width="2"
            marker-end=format!("url(#{})", data.get().key)
            on:click=move |_| {
                onClick();
            }
        >
        </line>
    }
}
