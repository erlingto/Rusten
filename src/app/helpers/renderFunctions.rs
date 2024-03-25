use leptos::{RwSignal, SignalGet};
use leptos_use::core::Position;
use web_sys::{js_sys::Math, wasm_bindgen::JsValue, CanvasRenderingContext2d};

use crate::app::{
    components::connection::LinePosition,
    structs::{connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem},
};

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
fn drawArrowhead(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    context: &CanvasRenderingContext2d,
    colorString: &str,
) {
    let angle = Math::atan2(y2 - y1, x2 - x1);
    context.save();
    context.begin_path();
    context.translate(x2, y2);
    context.rotate(angle);
    context.move_to(0.0, 0.0);
    context.line_to(-8.0, 4.0);
    context.line_to(-8.0, -4.0);
    context.close_path();
    context.set_fill_style(&JsValue::from_str(colorString));
    context.fill();
    context.restore();
}

fn renderConnectionLine(
    connection: ConnectionItem,
    context: &CanvasRenderingContext2d,
    mousePosition: Position,
) {
    let line_position = calculateFromAndTo(connection.from.get(), connection.to.get());
    let mut color = "black";
    if (isMouseOverConnection(connection, mousePosition)) {
        color = "red";
    }

    context.set_stroke_style(&JsValue::from_str(color));
    context.begin_path();
    context.move_to(line_position.x1, line_position.y1);
    context.line_to(line_position.x2, line_position.y2);
    context.stroke();
    drawArrowhead(
        line_position.x1,
        line_position.y1,
        line_position.x2,
        line_position.y2,
        context,
        color,
    );
}

pub fn renderConnectionLines(
    connections: Vec<RwSignal<ConnectionItem>>,
    context: &CanvasRenderingContext2d,
    mousePosition: Position,
) {
    connections.iter().for_each(|x| {
        renderConnectionLine(x.get(), context, mousePosition);
    });
}

fn isMouseOverConnection(connection: ConnectionItem, mousePosition: Position) -> bool {
    let threshold = 5.0;
    let line_position = calculateFromAndTo(connection.from.get(), connection.to.get());
    let fromx = line_position.x1;
    let fromy = line_position.y1;
    let tox = line_position.x2;
    let toy = line_position.y2;

    let distance = Math::sqrt(Math::pow(tox - fromx, 2.0) + Math::pow(toy - fromy, 2.0));
    let distanceToMouse =
        Math::sqrt(Math::pow(tox - mousePosition.x, 2.0) + Math::pow(toy - mousePosition.y, 2.0))
            + Math::sqrt(
                Math::pow(fromx - mousePosition.x, 2.0) + Math::pow(fromy - mousePosition.y, 2.0),
            );
    distanceToMouse - distance < threshold
}

fn calculateFromAndTo(from: MoveBoxItem, to: MoveBoxItem) -> LinePosition {
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
}
