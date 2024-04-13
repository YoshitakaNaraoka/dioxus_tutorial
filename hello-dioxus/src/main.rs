use dioxus::prelude::*;
use std::thread::Scope;

fn main() {
    launch{app};
}

fn app(cx: Scope) -> Element {
    render! {
        div {
            "Hello, world!"
        }
    }
}