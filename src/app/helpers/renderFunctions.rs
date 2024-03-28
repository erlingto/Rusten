use leptos::{RwSignal, SignalGet};
use leptos_use::core::Position;
use web_sys::{js_sys::Math, wasm_bindgen::JsValue, CanvasRenderingContext2d, DomRect};

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
    DomRect: &DomRect,
) {
    let mut line_position = calculateFromAndTo(connection.from.get(), connection.to.get());
    line_position = LinePosition {
        x1: line_position.x1 - DomRect.left(),
        y1: line_position.y1 - DomRect.top(),
        x2: line_position.x2 - DomRect.left(),
        y2: line_position.y2 - DomRect.top(),
    };
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
    newConnectionStart: Option<leptos::RwSignal<MoveBoxItem>>,
    connections: Vec<RwSignal<ConnectionItem>>,
    context: &CanvasRenderingContext2d,
    mousePosition: Position,
    DomRect: &DomRect,
) {

    if let Some(newConnectionStart) = newConnectionStart {
        let positionFrom = newConnectionStart.get().position.get();
        let fromSize = newConnectionStart.get().size.get();
        let x1 = positionFrom.x + fromSize.x / 2.0 - DomRect.left();
        let y1 = positionFrom.y + fromSize.y / 2.0 - DomRect.top();
        let x2 = mousePosition.x - DomRect.left();
        let y2 = mousePosition.y - DomRect.top();
        context.set_stroke_style(&JsValue::from_str("black"));
        context.begin_path();
        context.move_to(x1, y1);
        context.line_to(x2, y2);
        context.stroke();
        drawArrowhead(x1, y1, x2, y2, context, "black");
    }

    connections.iter().for_each(|x| {
        renderConnectionLine(x.get(), context, mousePosition, DomRect);
    });
}

fn distance_to_point(line_position: LinePosition, x3: f64, y3: f64) -> f64 {
    let x1 = line_position.x1;
    let y1 = line_position.y1;
    let x2 = line_position.x2;
    let y2 = line_position.y2;

    let line_length_sq = (x2 - x1).powi(2) + (y2 - y1).powi(2);
    if line_length_sq == 0.0 {
        return (x3 - x1).hypot(y3 - y1);
    }
    let t = ((x3 - x1) * (x2 - x1) + (y3 - y1) * (y2 - y1)) / line_length_sq;
    let t = t.max(0.0).min(1.0);

    let closest_x = x1 + t * (x2 - x1);
    let closest_y = y1 + t * (y2 - y1);

    ((x3 - closest_x).powi(2) + (y3 - closest_y).powi(2)).sqrt()
}


pub fn isMouseOverConnection(connection: ConnectionItem, mousePosition: Position) -> bool {
    let threshold = 5.0;
    let line_position = calculateFromAndTo(connection.from.get(), connection.to.get());
    let fromx = line_position.x1;
    let fromy = line_position.y1;
    let tox = line_position.x2;
    let toy = line_position.y2;
    let distanceToMouse = distance_to_point(line_position, mousePosition.x, mousePosition.y);
    distanceToMouse < threshold
}

fn calculateFromAndTo(from: MoveBoxItem, to: MoveBoxItem) -> LinePosition {
    let fromPosition = from.position.get();
    let toPosition = to.position.get();
    let fromSize = from.size.get();
    let toSize = to.size.get();
    let fromX = fromPosition.x + fromSize.x / 2.0;
    let fromY = fromPosition.y + fromSize.y / 2.0;
    let closest_x = max(
        (toPosition.x),
        min(fromX, toPosition.x + toSize.x + 5.0),
    );
    let closest_y = max(
        (toPosition.y),
        min(fromY, toPosition.y + toSize.y + 5.0),
    );

    LinePosition {
        x1: fromX,
        y1: fromY,
        x2: closest_x,
        y2: closest_y,
    }
}
