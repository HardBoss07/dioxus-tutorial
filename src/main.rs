mod dog;

use dioxus::prelude::*;
use dog::DogApp;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let dogs = ["Matt", "Nina", "Roscoe"];
    rsx! {
        h1 { "Hot Dogs!" }
        span { "An App for saving cute pictures of dogs." }
        for dog in dogs {
            li { DogApp { name: dog } }
        }
    }
}