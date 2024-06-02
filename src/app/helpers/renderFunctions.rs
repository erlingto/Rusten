use leptos::{RwSignal, SignalGet};
use leptos_use::core::Position;

use web_sys::{js_sys::Math, wasm_bindgen::JsValue, CanvasRenderingContext2d, DomRect};

use crate::app::structs::{
    connectionItem::ConnectionItem, linePosition::LinePosition, moveBoxItem::MoveBoxItem,
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
fn draw_arrowhead(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    context: &CanvasRenderingContext2d,
    color_string: &str,
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
    context.set_fill_style(&JsValue::from_str(color_string));
    context.fill();
    context.restore();
}

fn render_connection_line(
    scale: f64,
    connection: ConnectionItem,
    context: &CanvasRenderingContext2d,
    mouse_position: Position,
    dom_rect: &DomRect,
) {
    let mut line_position =
        calculate_from_and_to(scale, connection.from.get(), connection.to.get());
    line_position = LinePosition {
        x1: line_position.x1 - dom_rect.left(),
        y1: line_position.y1 - dom_rect.top(),
        x2: line_position.x2 - dom_rect.left(),
        y2: line_position.y2 - dom_rect.top(),
    };
    let mut color = "black";
    if is_mouse_over_connection(scale, connection, mouse_position) {
        color = "red";
    }
    let lineWidth = 2.0;
    context.set_line_width(lineWidth);
    context.set_stroke_style(&JsValue::from_str(color));
    context.begin_path();
    context.move_to(line_position.x1, line_position.y1);
    context.line_to(line_position.x2, line_position.y2);
    context.stroke();
    draw_arrowhead(
        line_position.x1,
        line_position.y1,
        line_position.x2,
        line_position.y2,
        context,
        color,
    );
}

pub fn render_grid(
    context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    scale: f64,
    offsetX: f64,
    offsetY: f64,
    strokeStyle: &str,
    lineWidth: f64,
    cellSize: f64,
) {
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str(strokeStyle));
    context.set_line_width(lineWidth);
    context.begin_path();

    for i in 0..((width / scale) as i32 / cellSize as i32 + 1) {
        let mut x = (offsetX % cellSize) * scale;
        x += i as f64 * cellSize * scale;
        context.move_to(x, 0.0);
        context.line_to(x, height);
    }
    for i in 0..((height / scale) as i32 / cellSize as i32 + 1) {
        let mut y = (offsetY % cellSize) * scale;
        y += i as f64 * cellSize * scale;
        context.move_to(0.0, y);
        context.line_to(width, y);
    }
    context.stroke();
}

pub fn render_connection_lines(
    new_connection_start: Option<leptos::RwSignal<MoveBoxItem>>,
    connections: Vec<RwSignal<ConnectionItem>>,
    context: &CanvasRenderingContext2d,
    mouse_position: Position,
    scale: f64,
    dom_rect: &DomRect,
) {
    if let Some(new_connection_start) = new_connection_start {
        let position_from = new_connection_start.get().position.get();
        let from_size = new_connection_start.get().size.get();
        let x1 = position_from.x + from_size.x * scale / 2.0 - dom_rect.left();
        let y1 = position_from.y + from_size.y * scale / 2.0 - dom_rect.top();
        let x2 = mouse_position.x - dom_rect.left();
        let y2 = mouse_position.y - dom_rect.top();
        context.set_stroke_style(&JsValue::from_str("black"));
        context.begin_path();
        context.move_to(x1, y1);
        context.line_to(x2, y2);
        context.stroke();
        draw_arrowhead(x1, y1, x2, y2, context, "black");
    }

    connections.iter().for_each(|x| {
        render_connection_line(scale, x.get(), context, mouse_position, dom_rect);
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

pub fn is_mouse_over_connection(
    scale: f64,
    connection: ConnectionItem,
    mouse_position: Position,
) -> bool {
    let threshold = 5.0;
    let line_position = calculate_from_and_to(scale, connection.from.get(), connection.to.get());
    let distance_to_mouse = distance_to_point(line_position, mouse_position.x, mouse_position.y);
    distance_to_mouse < threshold
}

fn calculate_from_and_to(scale: f64, from: MoveBoxItem, to: MoveBoxItem) -> LinePosition {
    let from_position = from.position.get();
    let to_position = to.position.get();
    let from_size = from.size.get();
    let to_size = to.size.get();
    let from_x = from_position.x + from_size.x * scale / 2.0;
    let from_y = from_position.y + from_size.y * scale / 2.0;
    let closest_x = max(
        to_position.x,
        min(from_x, to_position.x + to_size.x * scale + 5.0),
    );
    let closest_y = max(
        to_position.y,
        min(from_y, to_position.y + to_size.y * scale + 5.0),
    );

    LinePosition {
        x1: from_x,
        y1: from_y,
        x2: closest_x,
        y2: closest_y,
    }
}

pub fn shouldRender(position: Position, size: Position, width: f64, height: f64) -> bool {
    let padding = 2.0;
    let xInBounds = position.x + size.x + padding >= 0.0 && position.x <= width + padding;
    let yInBounds = position.y + size.y + padding >= 0.0 && position.y <= height + padding;
    xInBounds && yInBounds
}
