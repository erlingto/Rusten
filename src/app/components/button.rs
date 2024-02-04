use leptos::*;
#[component]
pub fn Button<F: Fn() -> () + 'static>(
    onClick: F,
    title: String,
    signal: Signal<String>,
) -> impl IntoView {
    view! {
        <button
            style="padding: 20px"
            on:click=move |_| {
                onClick();
            }
        >

            {title}
            ":"
            {signal}
        </button>
    }
}
