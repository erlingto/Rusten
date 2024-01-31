use crate::app::App;
pub mod app;
use leptos::*;
use log::Level;
use log::debug;

fn main() {
    console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    debug!("Starting app...");
    mount_to_body(|| view! { <App/> })
}
