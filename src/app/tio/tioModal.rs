use crate::app::tio::tioCard::TioCard;
use leptos::html::Div;
use leptos::*;
use leptos_use::on_click_outside;

#[component]
pub fn TioModal(children: Children) -> impl IntoView {
    view! {
        <dialog backdrop="background:red" show_modal=true open=true class="modal">
            <div class="inner">
                <TioCard>{children()}</TioCard>
            </div>
        </dialog>

        <style>
            "
            .modal {
            position: fixed;
            left: 40%;
            top: 20%;
            transform: translate(-50%, -50%);
            width: 420px;
            max-width: 100%;
            z-index: 1000;
            backdrop-filter: blur(1px);
            }
            .inner {
            padding: 0.4em 2em;
            }
            dialog::backdrop {
                background-color: salmon;
            }
            "
        </style>
    }
}
