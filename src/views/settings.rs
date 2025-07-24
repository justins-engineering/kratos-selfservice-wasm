use crate::components::FormBuilder;
use crate::{Configuration, Create, Route};
use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::{create_browser_settings_flow, get_settings_flow};

#[component]
pub fn Settings() -> Element {
  let create_flow = use_resource(move || async move {
    create_browser_settings_flow(&Configuration::create(), None, None).await
  });

  return match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        debug!("{res:#?}");
        rsx! {
          h1 { class: "text-center text-2xl", "User Settings" }
          div { class: "mx-auto w-full max-w-lg",
            FormBuilder { ui: *res.ui.to_owned() }
          }
        }
      }
      Err(err) => {
        rsx! {
          p { "Failed to create RegistrationFlow! Error: {err:?}" }
        }
      }
    },
    None => rsx! {},
  };
}

#[component]
pub fn SettingsFlow(flow: String) -> Element {
  let id = flow.clone();
  let get_flow = use_resource(move || {
    let id = id.to_owned();
    async move { get_settings_flow(&Configuration::create(), &id, None, None).await }
  });

  return match &*get_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        debug!("{res:#?}");
        rsx! {
          h1 { class: "text-center text-2xl", "User Settings" }
          div { class: "mx-auto w-full max-w-lg",
            FormBuilder { ui: *res.ui.to_owned() }
          }
        }
      }
      Err(err) => {
        navigator().replace(Route::SignUp {});
        rsx! {
          p { "Failed to get RegistrationFlow! Error: {err:?}" }
        }
      }
    },
    None => rsx! {},
  };
}
