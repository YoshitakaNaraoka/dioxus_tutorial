use dioxus::prelude::*;
use dioxus::launch;
use dioxus_web::launch::launch;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        div {
            "Hello, world!"
        }
    }
}