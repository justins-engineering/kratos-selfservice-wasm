use crate::components::{DisplayError, FormBuilder};
use crate::{Configuration, Create, Route};
use dioxus::logger::tracing::{debug, error};
use dioxus::prelude::*;
use ory_kratos_client_wasm::apis::frontend_api::{create_browser_login_flow, get_login_flow};

#[component]
pub fn SignIn() -> Element {
  let create_flow = use_resource(move || async move {
    create_browser_login_flow(
      &Configuration::create(),
      None,
      None,
      None,
      None,
      None,
      None,
      None,
      None,
    )
    .await
  });

  // refresh: Option<bool>, aal: Option<&str>, return_to: Option<&str>, cookie: Option<&str>,
  // login_challenge: Option<&str>, organization: Option<&str>, via: Option<&str>

  return match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        debug!("{res:#?}");
        rsx! {
          h1 { class: "text-center text-2xl", "Sign In" }
          div { class: "mx-auto w-full max-w-lg",
            div { class: "mt-10",
              FormBuilder { ui: *res.ui.to_owned() }
              p { class: "text-sm leading-6",
                "Don't have an account? "
                Link {
                  to: Route::SignUp {},
                  class: "link-primary link-hover",
                  "Get started →"
                }
              }
            }
          }
        }
      }
      Err(ory_kratos_client_wasm::apis::Error::ResponseError(res)) => rsx! {
        {res.to_owned().view_response_content()}
      },

      Err(err) => {
        error!("{err:#?}");
        rsx! {
          p { "Failed to get RegistrationFlow! Error:" }
          p { "{err:#?}" }
        }
      }
    },
    None => rsx! {},
  };
}

#[component]
pub fn LoginFlow(flow: String) -> Element {
  let get_flow = use_resource(move || {
    let id = flow.to_owned();
    async move { get_login_flow(&Configuration::create(), &id, None).await }
  });

  return match &*get_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        debug!("{res:#?}");
        rsx! {
          h1 { class: "text-center text-2xl", "Sign In" }
          div { class: "mx-auto w-full max-w-lg",
            div { class: "mt-10",
              FormBuilder { ui: *res.ui.to_owned() }
              p { class: "text-sm leading-6",
                "Don't have an account? "
                Link {
                  to: Route::SignUp {},
                  class: "link-primary link-hover",
                  "Get started →"
                }
              }
            }
          }
        }
      }
      Err(err) => {
        navigator().replace(Route::SignIn {});
        rsx! {
          p { "Failed to get LoginFlow! Error: {err:?}" }
        }
      }
    },
    None => rsx! {},
  };
}
