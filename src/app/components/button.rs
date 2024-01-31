use leptos::*;
#[component]
pub fn Button<F: Fn() -> () + 'static>(onClick: F) -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button style="padding: 20px" on:click=move |_| {
            onClick();
            set_count(count.get() + 1);
        }>"Add MoveBox: " {move || count.get()}</button>
    }
}
