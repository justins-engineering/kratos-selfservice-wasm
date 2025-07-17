use crate::{Configuration, Create, Route};
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::get_flow_error;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
  rsx! {
    div { class: "text-center max-h-screen max-w-none",
      h1 { class: "text-9xl my-12", "404" }
      h2 { class: "text-2xl my-8", "Oops! Page not found." }
      h3 { class: "font-light my-8",
        "The page {route:?} might have been removed or is temporarily unavailable."
      }
      Link { to: Route::Home {}, class: "btn btn-primary my-8", "Go Home" }
    }
  }
}

#[component]
pub fn ServerError(id: String) -> Element {
  let err_id = id.clone().to_owned();
  let future = use_resource(move || {
    let err_id = err_id.to_owned();
    async move { get_flow_error(&Configuration::create(), &err_id).await }
  });

  rsx! {
    div { class: "text-center max-h-screen max-w-none",
      h1 { class: "text-2xl my-8", "Oops! We've encountered an error." }
      h2 { class: "font-light my-8", "Error ID: {id}" }

      match &*future.read_unchecked() {
          Some(Ok(res)) => {
              match &res.error {
                  Some(Value) => {
                      if Value.is_object() {
                          rsx! {
                            for (key , value) in Value.as_object().unwrap() {
                              p { class: "font-light m-8", "{key}: {value}" }
                            }
                          }
                      } else {
                          rsx! {
                            p { class: "font-light m-8", "Error: {Value:?}" }
                          }
                      }
                  }
                  None => rsx! {},
              }
          }
          Some(Err(err)) => rsx! {
            p { class: "font-light m-8", "Failed to get error: {err}" }
          },
          None => rsx! {
            p { class: "font-light my-8", "Fetching error..." }
          },
      }
      Link { to: Route::Home {}, class: "btn btn-primary my-8", "Go Home" }
    }
  }
}
