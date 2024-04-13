use dioxus::prelude::*;
use dioxus::launch;
use dioxus_web::launch::launch;
use std::thread::Scope;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        div {
            "Hello, world!"
        }
    }
}