use crate::components::FormBuilder;
use crate::{Configuration, Create, Route};
use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::{
  create_browser_verification_flow, get_verification_flow,
};

#[component]
pub fn Verify() -> Element {
  let create_flow = use_resource(move || async move {
    create_browser_verification_flow(&Configuration::create(), None).await
  });

  return match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        debug!("{res:#?}");
        rsx! {
          h1 { class: "text-center text-2xl", "Account Verification" }
          div { class: "mx-auto w-full max-w-lg",
            div { class: "mt-10",
              FormBuilder { ui: *res.ui.to_owned() }
              p { class: "text-sm leading-6",
                "Already have an account? "
                Link {
                  to: Route::SignIn {},
                  class: "link-primary link-hover",
                  "Login →"
                }
              }
            }
          }
        }
      }
      Err(err) => {
        rsx! {
          p { "Failed to create VerificationFlow! Error: {err:?}" }
        }
      }
    },
    None => rsx! {},
  };
}

#[component]
pub fn VerificationFlow(flow: String) -> Element {
  let id = flow.clone();
  let get_flow = use_resource(move || {
    let id = id.to_owned();
    async move { get_verification_flow(&Configuration::create(), &id, None).await }
  });

  return match &*get_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        debug!("{res:#?}");
        rsx! {
          h1 { class: "text-center text-2xl", "Account Verification" }
          div { class: "mx-auto w-full max-w-lg",
            div { class: "mt-10",
              FormBuilder { ui: *res.ui.to_owned() }
              p { class: "text-sm leading-6",
                "Already have an account? "
                Link {
                  to: Route::SignIn {},
                  class: "link-primary link-hover",
                  "Login →"
                }
              }
            }
          }
        }
      }
      Err(err) => {
        navigator().replace(Route::Verify {});
        rsx! {
          p { "Failed to get VerificationFlow! Error: {err:?}" }
        }
      }
    },
    None => rsx! {},
  };
}
