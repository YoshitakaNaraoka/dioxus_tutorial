use dioxus::prelude::*;
use std::thread::Scope;

fn main() {
    launch(app);
}

#[warn(deprecated)]
fn app(cx: Scope) -> Element {
    render! {
        div {
            "Hello, world!"
        }
    }
}