use crate::components::FormBuilder;
use crate::{Configuration, Create, Route};
use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::{create_browser_login_flow, get_login_flow};

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
    )
    .await
  });

  // refresh: Option<bool>, aal: Option<&str>, return_to: Option<&str>, cookie: Option<&str>,
  // login_challenge: Option<&str>, organization: Option<&str>, via: Option<&str>

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
                    legend { class: "fieldset-legend text-2xl", "Sign In" }
                    FormBuilder { nodes: res.ui.nodes.to_owned() }
                  }
                }
              }
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
        rsx! {
          h1 { "Failed to create LoginFlow! Error: {err:?}" }
        }
      }
    },
    None => rsx! {},
  };
}

#[component]
pub fn LoginFlow(flow: String) -> Element {
  let id = flow.clone();
  let get_flow = use_resource(move || {
    let id = id.to_owned();
    async move { get_login_flow(&Configuration::create(), &id, None).await }
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
                    legend { class: "fieldset-legend text-2xl", "Sign In" }
                    FormBuilder { nodes: res.ui.nodes.to_owned() }
                  }
                }
              }
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
          h1 { "Failed to get LoginFlow! Error: {err:?}" }
        }
      }
    },
    None => rsx! {},
  };
}
