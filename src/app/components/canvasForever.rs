use crate::app::components::connection::Connection;
use crate::app::components::diagramTextBox::DiagramTextBox;
use crate::app::components::move_box::MoveBox;
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use crate::app::tio::tioButton::TioButton;
use leptos::ev::{dragend, dragstart, touchend, touchstart};
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_event_listener, use_mouse, UseMouseReturn};
use log::debug;
use styled::tracing::event;

#[component]
pub fn CanvasForever(
    moveBoxes: Vec<RwSignal<MoveBoxItem>>,
    connections: Vec<RwSignal<ConnectionItem>>,
    mouseConnection: RwSignal<MoveBoxItem>,
) -> impl IntoView {
    //let width = document().().unwrap().as_f64().unwrap() as i32;
    //let height = document().inner_height().unwrap().as_f64().unwrap() as i32;

    let width = 800.0;
    let height = 950.0;

    let startX = create_rw_signal(0.0);
    let startY = create_rw_signal(0.0);
    let scale = create_rw_signal(1.0);

    let offsetX = create_rw_signal(0.0);
    let offsetY = create_rw_signal(0.0);

    let toVirtualX = |xReal: f64| -> f64 { xReal / width as f64 * scale.get() };
    let toVirtualY = |yReal: f64| -> f64 { yReal / height as f64 * scale.get() };

    let toRealX = |xVirtual: f64| -> f64 { xVirtual / scale.get() * width as f64 };
    let toRealY = |yVirtual: f64| -> f64 { yVirtual / scale.get() * height as f64 };

    let virtualHeight = move || height / scale.get();
    let virtualWidth = move || width / scale.get();

    let UseMouseReturn {
        x: xReal,
        y: yReal,
        source_type,
        ..
    } = use_mouse();

    let handleDragStart = move |event: web_sys::DragEvent| {
        let x = event.client_x();
        let y = event.client_y();
        startX.set(x.clone() as f64);
        startY.set(y.clone() as f64);
    };

    let handleDragEnd = move |event: web_sys::DragEvent| {
        let x = event.client_x() as f64;
        let y = event.client_y() as f64;

        let distanceX = x - startX.get();
        let distanceY = y - startY.get();

        offsetX.set(distanceX / scale.get());
        offsetX.set(distanceY / scale.get());

        //let prevMidX = (startX.get() + x as f64) / 2.0;
        //let prevMidY = (startY.get() + y as f64) / 2.0;

        let xReal = x.clone() as f64;
        let yReal = y.clone() as f64;
        let xVirtual = toVirtualX(xReal);
        let yVirtual = toVirtualY(yReal);
        let xStart = toVirtualX(startX.get());
        let yStart = toVirtualY(startY.get());
        let xEnd = toVirtualX(xReal);
        let yEnd = toVirtualY(yReal);

        startX.set(x.clone() as f64);
        startY.set(y.clone() as f64);
    };

    let zoom = move |event: web_sys::WheelEvent| {
        let deltay = event.delta_y();
        let deltax = event.delta_x();
        scale.set(scale.get() + deltay / 100.0);
    };

    let drawGrid = move |canvas: web_sys::HtmlCanvasElement| {
        let strokeStyle = "rgb(229,231,235)";
        let lineWidth = 1.0;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.style().set_property("strokeStyle", strokeStyle);
        context.style().set_property("lineWidth", lineWidth);
        context.begin_path();

        for i in 0..(offsetX.get() % 5 as i32) {
            let x = toRealX(i as f64);
            context.move_to(x, 0.0);
            context.line_to(x, height as f64);
        }
    };

    use_event_listener(canvas, dragstart, handleDragStart);
    use_event_listener(canvas, dragend, handleDragEnd);

    let touchStartEventListener = |event: web_sys::TouchEvent| {};

    view! { <canvas></canvas> }
}
