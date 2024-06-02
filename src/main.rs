#![feature(get_many_mut)]
use crate::app::App;
pub mod app;
use leptos::*;
use log::debug;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();
    debug!("Starting app...");
    SpecialNonReactiveZone::enter();
    mount_to_body(|| view! { <App/> })
}
