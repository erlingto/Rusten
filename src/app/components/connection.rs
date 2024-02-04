use super::structs::ConnectionItem;
use leptos::*;
use log::debug;

pub struct LinePosition {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[component]
pub fn Connection<F: Fn() -> () + 'static>(onClick: F, data: ConnectionItem) -> impl IntoView {
    let fromX = create_rw_signal(data.from.position.get().x);
    let toX = create_rw_signal(data.to.position.get().x);
    let fromY = create_rw_signal(data.from.position.get().y);
    let toY = create_rw_signal(data.to.position.get().y);

    create_effect(move |_| {
        let newFrom = data.from.position.get();
        let newTo = data.to.position.get();
        if (newFrom.x != fromX.get() || newFrom.y != fromY.get()) {
            fromX.set(newFrom.x);
            fromY.set(newFrom.y);
        }
        if (newTo.x != toX.get() || newTo.y != toY.get()) {
            toX.set(newTo.x);
            toY.set(newTo.y);
            debug!("NewTo: {:?}", newTo);
        }
    });

    view! {
        <line
            position="absolute"
            id=data.key.clone()
            x1=fromX
            y1=fromY
            x2=toX
            y2=toY
            style="stroke:rgb(0,0,0);stroke-width:2"
        ></line>
    }
}
