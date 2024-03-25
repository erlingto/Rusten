use crate::app::components::connection::Connection;
use crate::app::components::move_box::MoveBox;
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use leptos::html::Canvas;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_mouse, UseMouseReturn};

use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsValue;

#[component]
pub fn CanvasForever(
    new_connection_start: RwSignal<Option<RwSignal<MoveBoxItem>>>,
    connections: RwSignal<Vec<RwSignal<ConnectionItem>>>,
    items: RwSignal<Vec<RwSignal<MoveBoxItem>>>,
    is_connecting: RwSignal<bool>,
) -> impl IntoView {
    let mounted = create_rw_signal(false);

    let width = document().body().unwrap().client_width() as f64;
    let height = width / 2.0;
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

    let toRealX = move |xVirtual: f64| -> f64 { xVirtual - offsetX.get() };
    let toRealY = move |yVirtual: f64| -> f64 { yVirtual - offsetY.get() };

    let virtualHeight = move || height / scale.get();
    let virtualWidth = move || width / scale.get();

    let canvasRect = create_rw_signal(None::<web_sys::DomRect>);

    let UseMouseReturn {
        x: xReal, y: yReal, ..
    } = use_mouse();

    let drawGrid = move |canvasref: NodeRef<Canvas>, offsetX: f64, offsetY: f64, scale: f64| {
        let cellSize = 100.0;
        let strokeStyle = "rgb(200,0,0)";
        let lineWidth = 1.0;
        let canvas = canvasref.get();
        if (canvas.is_some()) {
            let mounted_canvas_rect = canvas.clone().unwrap().get_bounding_client_rect();
            canvasRect.set(Some(mounted_canvas_rect));
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
        if (startDrag.get()) {
            let x = xReal.get() as f64;
            let y = yReal.get() as f64;
            let distanceX = x - startX.get();
            let distanceY = y - startY.get();
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

    let shouldRender = move |position: Position| -> bool {
        let x = position.x;
        let y = position.y;
        let xReal = toRealX(x);
        let yReal = toRealY(y);
        let xVirtual = toVirtualX(xReal);
        let yVirtual = toVirtualY(yReal);
        let virtualWidth = virtualWidth();
        let virtualHeight = virtualHeight();
        let xInBounds = xVirtual >= 0.0 && xVirtual <= virtualWidth;
        let yInBounds = yVirtual >= 0.0 && yVirtual <= virtualHeight;
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
            if (new_connection_start.get().is_none()) {
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

    let renderBoxes = create_effect(move |_| {
        SpecialNonReactiveZone::enter();
        for item in items.get().iter() {
            if (item.get().isDragging.get()) {
                continue;
            }
            let realPosition = item.get().realPosition.get();
            item.get().position.set(virtualPosition(realPosition));
        }
    });

    let getSvgStyle = move || {
        if (canvasRect.get().is_none()) {
            return format!(
                "position: absolute; top: {}px; left: {}px; width: 100%; height: 100%; z-index: -1;",
                0, 0
            );
        } else {
            format!(
                "position: absolute; top: {}px; left: {}px; width: 100%; height: 100%; z-index: -11;",
                canvasRect.get().unwrap().top(),
                canvasRect.get().unwrap().left()
            )
        }
    };

    view! {
        <div style="width:100%; height: 100%;">
            <div>Canvas</div>
            <canvas
                style=format!("width: {}px; height: {}px;", width, height)

                node_ref=canvasRef
            ></canvas>
            <For each=items key=|state| state.get().key.clone() let:child>
                <MoveBox
                    is_connecting=is_connecting
                    onClick=move || connect(child)
                    move_box_item=child
                />
            </For>
            <svg style=getSvgStyle>
                <Show when=move || new_connection_start.get().is_some() fallback=|| ()>
                    <line
                        position="absolute"
                        id="temp"
                        x1=new_connection_start.get().unwrap().get().position.get().x
                        y1=new_connection_start.get().unwrap().get().position.get().y
                        x2=xReal
                        y2=yReal
                        style="position: absolute ;stroke:rgb(0,0,0);stroke-width:2; z-index=1"
                    ></line>
                </Show>
                <For each=connections key=|state| state.get().key.clone() let:connection>
                    <Connection
                        onClick=move || {
                            let mut newConnections = connections.get();
                            newConnections.retain(|x| x.get().key != connection.get().key);
                            connections.set(newConnections);
                        }

                        data=connection
                    />
                </For>
            </svg>
            <div>
                offsetX: {offsetX} offsetY: {offsetY} scale: {scale} , mousePosition {xReal} ,
                {yReal}
            </div>
        </div>
    }
}
