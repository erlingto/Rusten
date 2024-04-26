use crate::app::tio::tioButton::TioButton;
use leptos::ChildrenFn;
use leptos::*;
#[component]
pub fn TioModal(children: ChildrenFn, show: RwSignal<bool>) -> impl IntoView {
    view! {
        <Show when=move || { show.get() }>
            <div style="position: fixed;
            z-index: 1;
            left: 0;
            top: 0;
            width: 80%;
            height: 100%;
            overflow: auto;
            background-color: rgba(0,0,0,0.4);">
                <div style=" background-color: #fefefe;
                margin: 15% auto;
                padding: 20px;
                border: 1px solid #888;
                width: 60%;">
                    <div style="position: relative">
                        <TioButton
                            on_click=move || {
                                show.set(false);
                            }

                            text=Signal::derive(move || "✕".to_string())
                            style="position: absolute; right: 1vw; background-color: #aaa; padding: 3px; padding-top: 0px; padding-bottom: 0px; border-radius: 5px; color: white; font-size: 20px;"
                                .to_string()
                        />
                    </div>
                    {children()}

                </div>

            </div>

        </Show>
    }
}
