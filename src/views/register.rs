use crate::{Configuration, Create, Route, KRATOS_BROWSER_URL};
use dioxus::logger::tracing::{debug, error};
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::{
  create_browser_registration_flow, update_registration_flow,
};

use crate::components::FormBuilder;

#[component]
pub fn Register(flow: String) -> Element {
  debug!("flow id: {flow}");

  // if flow.is_empty() {
  //   let nav = navigator();
  //   nav.replace(format!(
  //     "{}/self-service/registration/browser",
  //     KRATOS_BROWSER_URL
  //   ));
  // }

  // let new_flow: Signal<RegistrationFlow> = use_signal(|| RegistrationFlow::default());

  // if flow.is_empty() {
  let create_flow = use_resource(move || async move {
    create_browser_registration_flow(
      &Configuration::create(),
      None,
      None,
      Some("http://127.0.0.1:4455/registration"),
      None,
    )
    .await
  });

  return match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        rsx! {
          div { class: "mx-auto w-full max-w-sm",
            div { class: "mt-10",
              FormBuilder { flow: res.clone() }
              p { class: "text-sm leading-6",
                "Already have an account? "
                a {
                  class: "link-primary link-hover",
                  href: "http://127.0.0.1:4433/self-service/login/browser",
                  "Login →"
                }
              }
            }
          }
        }
      }
      Err(err) => {
        rsx! {
          h1 { "Failed to create RegistrationFlow! Error: {err}" }
        }
      }
    },
    None => rsx! {
      h1 { "Failed to create RegistrationFlow!" }
    },
  };
  // } else {
  //   let id = flow.clone();
  //   let get_flow = use_resource(move || {
  //     let id = id.to_owned();
  //     async move { get_registration_flow(&Configuration::create(), &id, None).await }
  //   });

  //   return match &*get_flow.read() {
  //     Some(new_flow) => match new_flow {
  //       Ok(res) => rsx! {
  //         PreRegister { flow: res.clone() }
  //       },
  //       Err(err) => rsx! {
  //         h1 { "Failed to get RegistrationFlow! Error: {err}" }
  //       },
  //     },
  //     None => rsx! {
  //       h1 { "Failed to get RegistrationFlow!" }
  //     },
  //   };
  // }
}
