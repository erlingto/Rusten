use crate::app::components::move_box::MoveBox;
use crate::app::helpers::renderFunctions::{
    is_mouse_over_connection, render_connection_lines, render_grid,
};
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use leptos::html::Canvas;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_mouse, UseMouseReturn};

use log::debug;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

#[component]
pub fn CanvasForever(
    new_connection_start: RwSignal<Option<RwSignal<MoveBoxItem>>>,
    connections: RwSignal<Vec<RwSignal<ConnectionItem>>>,
    items: RwSignal<Vec<RwSignal<MoveBoxItem>>>,
    is_connecting: RwSignal<bool>,
) -> impl IntoView {
    let mounted = create_rw_signal(false);

    let width = document().body().unwrap().client_width() as f64 * 0.8;
    let height = width / 1.8;
    let startDrag = create_rw_signal(false);
    let canvasRef = create_node_ref::<leptos::html::Canvas>();
    let scale = create_rw_signal(1.0);

    let startX = create_rw_signal(0.0);
    let startY = create_rw_signal(0.0);

    let cumuDistanceX = create_rw_signal(0.0);
    let cumuDistanceY = create_rw_signal(0.0);

    let offsetX = create_rw_signal(0.0);
    let offsetY = create_rw_signal(0.0);

    let toVirtualX = move |xReal: f64| -> f64 { xReal + offsetX.get() };
    let toVirtualY = move |yReal: f64| -> f64 { yReal + offsetY.get() };

    let toRealX = move |xVirtual: f64| -> f64 { xVirtual + offsetX.get() };
    let toRealY = move |yVirtual: f64| -> f64 { yVirtual + offsetY.get() };

    let virtualHeight = move || height / scale.get();
    let virtualWidth = move || width / scale.get();

    let canvasRect = create_rw_signal(None::<web_sys::DomRect>);

    let UseMouseReturn {
        x: xReal, y: yReal, ..
    } = use_mouse();

    let removeConnection = move |connection: RwSignal<ConnectionItem>| {
        let mut newConnections = connections.get();
        let index = newConnections
            .iter()
            .position(|x| x.get().key == connection.get().key)
            .unwrap();
        newConnections.remove(index);
        connections.set(newConnections);
    };

    let checkAndRemoveConnections = move || {
        let newConnections = connections.get();
        newConnections.iter().for_each(|connection| {
            if is_mouse_over_connection(
                connection.get(),
                Position {
                    x: xReal.get(),
                    y: yReal.get(),
                },
            ) {
                removeConnection(*connection);
            }
        });
    };

    let renderCanvas = move |canvasref: NodeRef<Canvas>, offsetX: f64, offsetY: f64, scale: f64| {
        let cellSize = 100.0;
        let strokeStyle = "rgb(0,0,0)";
        let lineWidth = 1.0;
        let canvas = canvasref.get();
        if canvas.is_some() {
            let mounted_canvas_rect = canvas.clone().unwrap().get_bounding_client_rect();
            let context = canvas
                .unwrap()
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            context.clear_rect(0.0, 0.0, width, height);

            render_connection_lines(
                new_connection_start.get(),
                connections.get(),
                &context,
                Position {
                    x: xReal.get(),
                    y: yReal.get(),
                },
                &mounted_canvas_rect,
            );
            render_grid(
                &context,
                Position {
                    x: xReal.get(),
                    y: yReal.get(),
                },
                &mounted_canvas_rect,
                width,
                height,
                scale,
                offsetX,
                offsetY,
                strokeStyle,
                lineWidth,
                cellSize,
            );
            canvasRect.set(Some(mounted_canvas_rect));
        }
    };

    let handleStart = move |_: web_sys::MouseEvent| {
        let x = xReal.get() as f64;
        let y = yReal.get() as f64;
        startX.set(x.clone() as f64);
        startY.set(y.clone() as f64);
        startDrag.set(true);
    };

    let handleMove = move |_: web_sys::MouseEvent| {
        if startDrag.get() {
            let x = xReal.get() as f64;
            let y = yReal.get() as f64;
            let distanceX = x - startX.get();
            let distanceY = y - startY.get();
            offsetX.set(cumuDistanceX.get() + distanceX / scale.get());
            offsetY.set(cumuDistanceY.get() + distanceY / scale.get());
        }
    };

    let handleEnd = move |_: web_sys::MouseEvent| {
        startDrag.set(false);
        cumuDistanceX.set(offsetX.get());
        cumuDistanceY.set(offsetY.get());
    };

    let _ = create_effect(move |_| {
        renderCanvas(canvasRef, offsetX.get(), offsetY.get(), scale.get());
    });

    let dragStartEL = move |canvasref: NodeRef<Canvas>| {
        let canvas = canvasref.get();
        if canvas.is_some() {
            let canvas = canvas.unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                handleStart(event);
            }) as Box<dyn FnMut(_)>);
            let _ = canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };

    let dragEndEL = move |canvasref: NodeRef<Canvas>| {
        let canvas = canvasref.get();
        if canvas.is_some() {
            let canvas = canvas.unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                handleEnd(event);
            }) as Box<dyn FnMut(_)>);
            let _ = canvas
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };

    let dragMoveEL = move |canvasref: NodeRef<Canvas>| {
        let canvas = canvasref.get();
        if canvas.is_some() {
            let canvas = canvas.unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                handleMove(event);
            }) as Box<dyn FnMut(_)>);
            let _ = canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };
    let _ = create_effect(move |_| {
        if canvasRef.get().is_some() && mounted.get() == false {
            let canvas = canvasRef.get().unwrap();
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);
            renderCanvas(canvasRef, offsetX.get(), offsetY.get(), scale.get());
            dragEndEL(canvasRef);
            dragMoveEL(canvasRef);
            dragStartEL(canvasRef);
            mounted.set(true);
        }
    });

    let shouldRender = move |position: Position| -> bool {
        let x = toRealX(position.x);
        let y = toRealY(position.y);
        let virtualWidth = virtualWidth();
        let virtualHeight = virtualHeight();
        let xInBounds = x >= 0.0 && x <= virtualWidth;
        let yInBounds = y >= 0.0 && y <= virtualHeight;
        return xInBounds && yInBounds;
    };

    let virtualPosition = move |position: Position| -> Position {
        let x = position.x;
        let y = position.y;
        let xVirtual = toVirtualX(x);
        let yVirtual = toVirtualY(y);
        return Position {
            x: xVirtual,
            y: yVirtual,
        };
    };

    let connect = move |moveBoxItem: RwSignal<MoveBoxItem>| {
        if is_connecting.get() {
            if new_connection_start.get().is_none() {
                new_connection_start.set(Some(moveBoxItem));
            } else {
                let from = new_connection_start.get().unwrap();
                let mut newConnections = connections.get();
                let newConnection = ConnectionItem {
                    key: connections.get().len().to_string(),
                    from: from,
                    to: moveBoxItem,
                };
                newConnections.push(create_rw_signal(newConnection));
                connections.set(newConnections);
                new_connection_start.set(None);
                is_connecting.set(false);
            }
        }
    };

    let _ = create_effect(move |_| {
        for item in items.get().iter() {
            if !shouldRender(item.get().realPosition.get()) {
                item.get().should_render.set(false);
            } else {
                item.get().should_render.set(true);
            }
            if item.get().isDragging.get() {
                continue;
            }
            let realPosition = item.get().realPosition.get();
            item.get().position.set(virtualPosition(realPosition));
        }
    });

    view! {
        <canvas
            style=format!("width: {}px; height: {}px; border: 1px solid black;", width, height)
            on:click=move |_| checkAndRemoveConnections()
            node_ref=canvasRef
        ></canvas>
        <For each=items key=|state| state.get().key.clone() let:child>
            <MoveBox
                is_connecting=is_connecting
                on_click=move || connect(child)
                move_box_item=child
            />
        </For>
        <div style="position: absolute; bottom: 0vh; height: 6vh; width: 90%; z-index:10; color : black; background-color: white;">
            offsetX: {offsetX} offsetY: {offsetY} scale: {scale} , mousePosition {xReal} , {yReal}
        </div>
    }
}
