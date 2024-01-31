use leptos::*;
use leptos::html::Div;
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};
use leptos_use::core::Position;
use log::debug;

#[component]
pub fn MoveBox(id: String, value: i32, position: Position) -> impl IntoView {
    let el = create_node_ref::<Div>();
    let (editable, setEditable) = create_signal(false);
    let (name, setName) = create_signal(String::from(value.to_string()));

    let modal_ref = create_node_ref::<Div>();

    // `style` is a helper string "left: {x}px; top: {y}px;"
    let UseDraggableReturn {
        x,
        y,
        style,
        ..
    } = use_draggable_with_options(
        el,
        UseDraggableOptions::default().initial_value(position),
    );


    fn ToggleEditable(editable: ReadSignal<bool>, setEditable: WriteSignal<bool>){
        debug!("ToggleEditable: {}", editable.get());
        setEditable(true);
    }


    
 
        view! {
            <Show when=move || editable.get() fallback=|| ()>
                <div node_ref=el style=move || format!("position: fixed; {} ;border:1px solid black;background-color: #EBEBEB", style.get() )>
                    <div class="inner">
                        <button
                            class="button small"
                            title="Close"
                            on:click=move |_| setEditable.set(false)
                        >
                            "𝖷"
                        </button>
                        <p class="heading">"Demo Modal"</p>
                        <p>"Click outside this modal to close it."</p>
                    </div>
                </div>
            </Show> 

            <Show when=move || !editable.get() fallback=|| ()>
                <div id={id.to_string()} node_ref=el style=move || format!("position: fixed; {} ;border:1px solid black;background-color: #EBEBEB", style.get() )>
                    <p>{value}</p>
                    <button on:click=move |_| {ToggleEditable(editable, setEditable)} >Edit</button>
                </div>
            </Show> 
        }
}