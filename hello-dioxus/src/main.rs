use dioxus::prelude::*;

fn main() {
    launch{app};
}

fn app(cx: Scope) -> Element {
    render {
        div {
            "Hello, world!"
        }
    }
}