use crate::components::DisplayError;
use crate::{Configuration, Create};
use dioxus::logger::tracing::{debug, error};
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::create_browser_logout_flow;

#[component]
pub fn OryLogOut() -> Element {
  let create_flow = use_resource(move || async move {
    create_browser_logout_flow(&Configuration::create(), None, None).await
  });

  return match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        rsx! {
          a { href: res.logout_url.clone(), "Log out" }
        }
      }
      Err(ory_kratos_client::apis::Error::ResponseError(res)) => {
        // error!("{res:#?}");
        rsx! {}
      }

      Err(err) => {
        error!("{err:#?}");
        rsx! {}
      }
    },
    None => rsx! {},
  };
}
