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
        debug!("{:#?}", res);
        rsx! {
          div { class: "mx-auto w-full max-w-sm",
            div { class: "mt-10",
              form {
                action: "{&res.ui.action}",
                method: "{&res.ui.method}",
                div { class: "mt-2",
                  fieldset { class: "fieldset",
                    legend { class: "fieldset-legend text-2xl", "Verify your account" }
                    FormBuilder { nodes: res.ui.nodes.to_owned() }
                  }
                }
              }
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
          h1 { "Failed to create VerificationFlow! Error: {err:?}" }
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
        debug!("{:#?}", res);
        rsx! {
          div { class: "mx-auto w-full max-w-sm",
            div { class: "mt-10",
              form {
                action: "{&res.ui.action}",
                method: "{&res.ui.method}",
                div { class: "mt-2",
                  fieldset { class: "fieldset",
                    legend { class: "fieldset-legend text-2xl", "Verify your account" }
                    p {
                      "An email containing a verification code has been sent to the email address you provided. If you have not received an email, check the spelling of the address and make sure to use the address you registered with."
                    }
                    FormBuilder { nodes: res.ui.nodes.to_owned() }
                  }
                }
              }
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
          h1 { "Failed to get VerificationFlow! Error: {err:?}" }
        }
      }
    },
    None => rsx! {},
  };
}
