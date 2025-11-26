use dioxus::prelude::*;

#[component]
pub fn DogApp(name: String) -> Element {
    rsx! {
        "Name: {name}"
    }
}