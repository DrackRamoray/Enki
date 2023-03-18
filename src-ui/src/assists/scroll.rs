use leptos::html::Ul;
use leptos::*;
use std::time::Duration;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

pub fn scroll_to_bottom(node_ref: NodeRef<Ul>) {
    set_timeout(
        move || {
            let el: HtmlElement<Ul> = node_ref.get().unwrap();
            let mut opts = ScrollIntoViewOptions::new();
            opts.behavior(ScrollBehavior::Smooth);
            el.scroll_into_view_with_scroll_into_view_options(&opts);
            window().scroll_to_with_x_and_y(0.0, 0.0);
        },
        Duration::from_millis(60),
    );
}
