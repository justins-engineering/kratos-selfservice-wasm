use crate::components::FormBuilder;
use crate::KRATOS_BROWSER_URL;
use dioxus::logger::tracing::{debug, error};
use dioxus::prelude::*;

use gloo_utils::format::JsValueSerdeExt;
use ory_kratos_client::apis::frontend_api::CreateBrowserRegistrationFlowError;
use ory_kratos_client::models::RegistrationFlow;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestCredentials, RequestInit, RequestMode, Response};

pub async fn create_browser_registration_flow(
  kratos_browser_url: &str,
) -> Result<RegistrationFlow, JsValue> {
  let opts = RequestInit::new();
  opts.set_method("GET");
  opts.set_mode(RequestMode::Cors);
  opts.set_credentials(RequestCredentials::Include);

  let url = format!("{}/self-service/registration/browser", kratos_browser_url);

  let request = Request::new_with_str_and_init(&url, &opts)?;

  request.headers().set("Accept", "application/json")?;

  let window = web_sys::window().unwrap();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

  // `resp_value` is a `Response` object.
  assert!(resp_value.is_instance_of::<Response>());
  let resp: Response = resp_value.dyn_into().unwrap();

  // Convert this other `Promise` into a rust `Future`.
  let json = JsFuture::from(resp.json()?).await?;

  Ok(json.into_serde().unwrap())
}

pub async fn get_registration_flow(
  kratos_browser_url: &str,
  id: &str,
  cookie: Option<&str>,
) -> Result<RegistrationFlow, JsValue> {
  let opts = RequestInit::new();
  opts.set_method("GET");
  opts.set_mode(RequestMode::Cors);
  opts.set_credentials(RequestCredentials::Include);

  let url = format!(
    "{}/self-service/registration/flows?id={}",
    kratos_browser_url, id
  );

  let request = Request::new_with_str_and_init(&url, &opts)?;

  request.headers().set("Accept", "application/json")?;

  if let Some(cookie) = cookie {
    request.headers().set("Cookie", cookie)?;
  }

  let window = web_sys::window().unwrap();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

  assert!(resp_value.is_instance_of::<Response>());
  let resp: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(resp.json()?).await?;
  debug!("{:?}", json);

  Ok(json.into_serde().unwrap())
}

#[component]
pub fn Register(flow: String) -> Element {
  if flow.is_empty() {
    let create_flow =
      use_resource(
        move || async move { create_browser_registration_flow(KRATOS_BROWSER_URL).await },
      );

    return match &*create_flow.read() {
      Some(new_flow) => match new_flow {
        Ok(res) => {
          debug!("{:?}", res);

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
            h1 { "Failed to create RegistrationFlow! Error: {err:?}" }
          }
        }
      },
      None => rsx! {
        h1 { "Failed to create RegistrationFlow!" }
      },
    };
  } else {
    debug!(flow);

    let id = flow.clone();
    let get_flow = use_resource(move || {
      let id = id.to_owned();
      async move { get_registration_flow(KRATOS_BROWSER_URL, &id, None).await }
    });

    return match &*get_flow.read() {
      Some(new_flow) => match new_flow {
        Ok(res) => {
          debug!("{:?}", res);

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
            h1 { "Failed to get RegistrationFlow! Error: {err:?}" }
          }
        }
      },
      None => rsx! {
        h1 { "Failed to get RegistrationFlow!" }
      },
    };
  }
}
