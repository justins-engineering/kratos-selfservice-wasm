use dioxus::prelude::*;

#[component]
pub fn Alert() -> Element {
  rsx! {
    div { role: "alert", class: "alert alert-info",
      span { "New software update available." }
    }
  }
}
