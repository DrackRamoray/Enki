#![feature(fn_traits)]

mod app;
mod assists;
mod components;
mod icons;
mod stores;
mod views;

use app::*;
use leptos::*;
use leptos_router::*;

pub fn mount() {
    mount_to_body(|cx| {
        view! { cx,
            <Router>
                <App/>
            </Router>
        }
    })
}
