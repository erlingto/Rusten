use crate::app::components::canvas::move_box::MoveBox;
use crate::app::helpers::renderFunctions::{
    is_mouse_over_connection, render_connection_lines, render_grid, shouldRender,
};
use crate::app::structs::connectionItem::ConnectionItem;
use crate::app::structs::moveBoxItem::MoveBoxItem;
use leptos::html::Canvas;
use leptos::*;
use leptos_use::core::Position;
use leptos_use::{use_document, use_element_hover, use_mouse, use_window, UseMouseReturn};

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
    let document = use_document();
    let window = use_window();
    let innerHeight = window
        .as_ref()
        .unwrap()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap();
    let width = document.body().unwrap().client_width() as f64 * 0.8;

    let height = innerHeight as f64 * 0.93;
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

    let _ = create_effect(move |_| {
        items.get().iter().for_each(|x| {
            let realPosition = x.get().realPosition;
            if !x.get().mounted.get() {
                realPosition.set(Position {
                    x: realPosition.get().x - offsetX.get(),
                    y: realPosition.get().y - offsetY.get(),
                });

                x.get().mounted.set(true);
            }
        });
    });

    let toVirtualPosition = move |position: Position| -> Position {
        let x = (position.x + offsetX.get()) * scale.get();
        let y = (position.y + offsetY.get()) * scale.get();
        Position { x, y }
    };

    let canvasRect = create_rw_signal(None::<web_sys::DomRect>);

    let UseMouseReturn {
        x: mouseX,
        y: mouseY,
        ..
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
                scale.get(),
                connection.get(),
                Position {
                    x: mouseX.get(),
                    y: mouseY.get(),
                },
            ) {
                removeConnection(*connection);
            }
        });
    };

    let renderCanvas = move |canvasref: NodeRef<Canvas>, offsetX: f64, offsetY: f64, scale: f64| {
        let cellSize = 100.0;
        let strokeStyle = "rgb(0,0,0, 0.2)";
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
                    x: mouseX.get(),
                    y: mouseY.get(),
                },
                scale,
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
        let x = mouseX.get_untracked();
        let y = mouseY.get_untracked();
        startX.set(x);
        startY.set(y);
        isDragging.set(true);
    };

    let handleMove = move |_: web_sys::MouseEvent| {
        if isDragging.get_untracked() {
            let x = mouseX.get_untracked();
            let y = mouseY.get_untracked();
            let distanceX = x - startX.get();
            let distanceY = y - startY.get();
            offsetX.set(cumuDistanceX.get_untracked() + distanceX / scale.get_untracked());
            offsetY.set(cumuDistanceY.get_untracked() + distanceY / scale.get_untracked());
        }
    };

    let handleScale = move |event: web_sys::WheelEvent| {
        event.prevent_default();
        let mut delta = event.delta_y();
        delta = delta / 1000.0 * 100.0;
        delta = delta.round() / 100.0;

        let newScale = ((scale.get_untracked() - delta) * 100.0).round() / 100.0;

        if newScale >= 0.2 {
            scale.set(newScale);
        } else {
            return;
        }

        let middleScale = scale.get() + delta / 2.0;

        let middleXCompensation =
            ((mouseX.get_untracked() + 1.0) / middleScale / middleScale) * delta;
        let middleYCompensation =
            ((mouseY.get_untracked() + 1.0) / middleScale / middleScale) * delta;

        let offsetChangeX = middleXCompensation;
        let offsetChangeY = middleYCompensation;

        offsetX.set(offsetX.get_untracked() + offsetChangeX);
        offsetY.set(offsetY.get_untracked() + offsetChangeY);
        reset_drag();
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
        if canvasRef.get().is_some() && !mounted.get() {
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
                    from,
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
            if !shouldRender(virtualPosition, item.size.get(), width, height) {
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
        <div style="position: absolute; top: 0.0vh; left: 0.0vh; height: 95vh; width: 0.3vw; z-index:10; color : black; background-color: white;"></div>
        <div style="position: absolute; left:0vw; bottom: 0vh; height: 6vh; width: 90%; z-index:10; color : black; background-color: white;">
            offsetX: {offsetX} offsetY: {offsetY} scale: {scale} , mousePosition {mouseX} , {mouseY}
        </div>
    }
}
