use crate::{Configuration, Create};
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use ory_kratos_client_wasm::apis::frontend_api::create_browser_logout_flow;

#[component]
pub fn OryLogOut() -> Element {
  let create_flow = use_resource(move || async move {
    create_browser_logout_flow(&Configuration::create(), None, None).await
  });

  return match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        rsx! {
          li {
            a { href: res.logout_url.to_owned(), "Log out" }
          }
        }
      }
      Err(err) => {
        error!("{err:#?}");
        rsx! {
          li { class: "menu-disabled",
            a { href: "", "Log out" }
          }
        }
      }
    },
    None => rsx! {
      li { class: "menu-disabled",
        a { href: "", "Log out" }
      }
    },
  };
}
