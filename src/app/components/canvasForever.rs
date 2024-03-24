use std::cell;

use crate::app::components::connection::Connection;
use crate::app::components::diagramTextBox::DiagramTextBox;
use crate::app::components::move_box::MoveBox;
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use crate::app::tio::tioButton::TioButton;
use leptos::ev::{dragend, dragstart, touchend, touchstart};
use leptos::html::Canvas;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_event_listener, use_mouse, UseMouseReturn};
use log::debug;
use styled::tracing::event;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

#[component]
pub fn CanvasForever() -> impl IntoView {
    //let width = document().().unwrap().as_f64().unwrap() as i32;
    //let height = document().inner_height().unwrap().as_f64().unwrap() as i32;

    let width = document().body().unwrap().client_width() as f64;
    let height = width / 2.0;

    debug!("width: {}, height: {}", width, height);
    let startDrag = create_rw_signal(false);

    let canvasRef = create_node_ref::<leptos::html::Canvas>();

    let startX = create_rw_signal(0.0);
    let startY = create_rw_signal(0.0);
    let scale = create_rw_signal(1.0);

    let cumuDistanceX = create_rw_signal(0.0);
    let cumuDistanceY = create_rw_signal(0.0);

    let offsetX = create_rw_signal(0.0);
    let offsetY = create_rw_signal(0.0);

    let toVirtualX = move |xReal: f64| -> f64 { xReal / width as f64 * scale.get() };
    let toVirtualY = move |yReal: f64| -> f64 { yReal / height as f64 * scale.get() };

    let toRealX = move |xVirtual: f64| -> f64 { xVirtual / scale.get() * width as f64 };
    let toRealY = move |yVirtual: f64| -> f64 { yVirtual / scale.get() * height as f64 };

    let virtualHeight = move || height / scale.get();
    let virtualWidth = move || width / scale.get();

    let mounted = create_rw_signal(false);

    let UseMouseReturn {
        x: xReal,
        y: yReal,
        source_type,
        ..
    } = use_mouse();

    let drawGrid = move |canvasref: NodeRef<Canvas>, offsetX: f64, offsetY: f64, scale: f64| {
        debug!("drawGrid");
        debug!(
            "offsetX: {}, offsetY: {}, scale: {}",
            offsetX, offsetY, scale
        );
        let cellSize = 100.0;
        let strokeStyle = "rgb(200,0,0)";
        let lineWidth = 1.0;
        let canvas = canvasref.get();
        if (canvas.is_some()) {
            debug!("canvasWidth: {}", canvas.clone().unwrap().width());
            let context = canvas
                .unwrap()
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            context.clear_rect(0.0, 0.0, width, height);
            context.begin_path();

            context.set_stroke_style(&JsValue::from_str(strokeStyle));
            context.set_line_width(lineWidth);
            context.begin_path();
            debug!("i: {}", width);
            for i in 0..(width as i32 / cellSize as i32 + 1) {
                let mut x = (offsetX % cellSize) * scale;
                x = x + i as f64 * cellSize * scale;
                context.move_to(x, 0.0);
                context.line_to(x, height as f64);
            }
            for i in 0..(height as i32 / cellSize as i32 + 1) {
                let mut y = (offsetY % cellSize) * scale;
                y = y + i as f64 * cellSize * scale;
                context.move_to(0.0, y);
                context.line_to(width as f64, y);
            }
            context.stroke();
        }
    };

    let handleStart = move |event: web_sys::MouseEvent| {
        let x = xReal.get() as f64;
        let y = yReal.get() as f64;
        startX.set(x.clone() as f64);
        startY.set(y.clone() as f64);
        startDrag.set(true);
    };

    let handleMove = move |event: web_sys::MouseEvent| {
        debug!("i: {}", startDrag.get());
        if (startDrag.get()) {
            let x = xReal.get() as f64;
            let y = yReal.get() as f64;
            debug!("x: {}", x);
            let distanceX = x - startX.get();
            let distanceY = y - startY.get();
            debug!("distanceX: {}", distanceX);
            offsetX.set(cumuDistanceX.get() + distanceX / scale.get());
            offsetY.set(cumuDistanceY.get() + distanceY / scale.get());
        }
    };

    let handleEnd = move |event: web_sys::MouseEvent| {
        startDrag.set(false);
        cumuDistanceX.set(offsetX.get());
        cumuDistanceY.set(offsetY.get());
    };

    let draw = create_effect(move |_| {
        drawGrid(canvasRef, offsetX.get(), offsetY.get(), scale.get());
    });

    let dragStartEL = move |canvasref: NodeRef<Canvas>| {
        let canvas = canvasref.get();
        if (canvas.is_some()) {
            let canvas = canvas.unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                handleStart(event);
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };

    let dragEndEL = move |canvasref: NodeRef<Canvas>| {
        let canvas = canvasref.get();
        if (canvas.is_some()) {
            let canvas = canvas.unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                handleEnd(event);
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };

    let dragMoveEL = move |canvasref: NodeRef<Canvas>| {
        let canvas = canvasref.get();
        if (canvas.is_some()) {
            let canvas = canvas.unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                handleMove(event);
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };
    let effect = create_effect(move |_| {
        if canvasRef.get().is_some() && mounted.get() == false {
            let canvas = canvasRef.get().unwrap();
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);
            drawGrid(canvasRef, offsetX.get(), offsetY.get(), scale.get());
            dragEndEL(canvasRef);
            dragMoveEL(canvasRef);
            dragStartEL(canvasRef);
            mounted.set(true);
        }
    });

    view! {
        <div style="widht:100%; height: 100%;">
            <div>Canvas</div>
            <canvas
                style=format!("width: {}px; height: {}px;", width, height)
                node_ref=canvasRef
            ></canvas>
            <div>
                offsetX: {offsetX} offsetY: {offsetY} scale: {scale} , mousePosition {xReal} ,
                {yReal}
            </div>
        </div>
    }
}
