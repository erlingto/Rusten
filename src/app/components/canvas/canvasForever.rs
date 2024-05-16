use crate::app::components::canvas::move_box::MoveBox;
use crate::app::helpers::renderFunctions::{
    is_mouse_over_connection, render_connection_lines, render_grid, shouldRender,
};
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use leptos::html::Canvas;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_element_hover, use_mouse, UseMouseReturn};

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
    let isDragging = create_rw_signal(false);
    let canvasRef = create_node_ref::<leptos::html::Canvas>();
    let scale = create_rw_signal(1.0);
    provide_context(scale);

    let startX = create_rw_signal(0.0);
    let startY = create_rw_signal(0.0);

    let cumuDistanceX = create_rw_signal(0.0);
    let cumuDistanceY = create_rw_signal(0.0);

    let offsetX = create_rw_signal(0.0);
    let offsetY = create_rw_signal(0.0);

    let toVirtualPosition = move |position: Position| -> Position {
        let x = (position.x + offsetX.get()) * scale.get();
        let y = (position.y + offsetY.get()) * scale.get();
        return Position { x, y };
    };

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

    let reset_drag = move || {
        isDragging.set(false);
        cumuDistanceX.set(offsetX.get_untracked());
        cumuDistanceY.set(offsetY.get_untracked());
    };

    let handleStart = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        let x = xReal.get_untracked() as f64;
        let y = yReal.get_untracked() as f64;
        startX.set(x.clone() as f64);
        startY.set(y.clone() as f64);
        isDragging.set(true);
    };

    let handleMove = move |_: web_sys::MouseEvent| {
        if isDragging.get_untracked() {
            let x = xReal.get_untracked() as f64;
            let y = yReal.get_untracked() as f64;
            let distanceX = x - startX.get();
            let distanceY = y - startY.get();
            offsetX.set(cumuDistanceX.get_untracked() + distanceX / scale.get_untracked());
            offsetY.set(cumuDistanceY.get_untracked() + distanceY / scale.get_untracked());
        }
    };

    let handleScale = move |event: web_sys::WheelEvent| {
        event.prevent_default();
        let delta = event.delta_y() as f64;
        let mut newScale = (scale.get_untracked() - delta / 1000.0) * 100.0;
        newScale = newScale.round() / 100.0;
        if newScale > 0.1 {
            scale.set(newScale);
        }
    };

    let handleEnd = move |_: web_sys::MouseEvent| reset_drag();
    let is_hovered = use_element_hover(canvasRef);
    let _ = create_effect(move |_| {
        if !is_hovered.get() {
            reset_drag()
        }
    });

    let _ = create_effect(move |_| {
        renderCanvas(canvasRef, offsetX.get(), offsetY.get(), scale.get());
    });

    let scaleEl = move |canvasref: NodeRef<Canvas>| {
        let canvas = canvasref.get();
        if canvas.is_some() {
            let canvas = canvas.unwrap();
            let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                handleScale(event);
            }) as Box<dyn FnMut(_)>);
            let _ =
                canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };

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
            scaleEl(canvasRef);
            mounted.set(true);
        }
    });

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
        for itemPtr in items.get().iter() {
            let item = itemPtr.get();
            let virtualPosition = toVirtualPosition(item.realPosition.get());
            if !shouldRender(
                virtualPosition,
                item.size.get(),
                virtualWidth(),
                virtualHeight(),
            ) {
                item.should_render.set(false);
            } else {
                item.should_render.set(true);
            }
            if item.isDragging.get() {
                continue;
            }
            item.position.set(virtualPosition);
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
                scale=scale
                is_connecting=is_connecting
                on_click=move || connect(child)
                move_box_item=child
            />
        </For>
        <div style="position: absolute; top: 0vh; height: 0.7vh; width: 95 vw; z-index:10; color : black; background-color: white;"></div>
        <div style="position: absolute; top: 0.7vh; left: 0vh; height: 95vh; width: 0.4vw; z-index:10; color : black; background-color: white;"></div>
        <div style="position: absolute; bottom: 0vh; height: 6vh; width: 90%; z-index:10; color : black; background-color: white;">
            offsetX: {offsetX} offsetY: {offsetY} scale: {scale} , mousePosition {xReal} , {yReal}
        </div>
    }
}
