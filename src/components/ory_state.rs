use std::error::Error;

use crate::{Configuration, Create, Route, Session, KRATOS_LIFETIME};
use chrono::{DateTime, Utc};
use dioxus::logger::tracing::{debug, error};
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::to_session;

#[cfg(feature = "web")]
use gloo_timers::callback::Timeout;

macro_rules! window {
  () => {
    web_sys::window().expect("Could not access window")
  };
}

macro_rules! document {
  ($window:expr) => {
    $window
      .document()
      .expect("Could not access window document")
  };
}

macro_rules! html_document {
  ($window:expr) => {
    web_sys::wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDocument>(
      $window
        .document()
        .expect("Could not access window document"),
    )
    .expect("Could not access HTMLDocument")
  };
}

macro_rules! get_cookies {
  ($html_document:expr) => {
    $html_document
      .cookie()
      .expect("Could not access HTMLDocument cookies")
  };
}

// pub fn set_session_state() {
//   let mut state = use_context::<Session>().state;

//   debug!("Setting Session state to true");
//   *state.write() = true;
//   let timeout = Timeout::new(KRATOS_LIFETIME, move || {
//     debug!("Setting Session state to false");
//     *state.write() = false;
//   });
//   timeout.forget();
// }

pub async fn use_session_state() -> Result<(bool, Option<gloo_timers::callback::Timeout>), String> {
  // let window = web_sys::window().expect("Could not access window");
  // let document = window.document().expect("Could not access window document");

  // let html_document = web_sys::wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDocument>(document)
  //   .expect("Could not access HTMLDocument");

  // let html_document = html_document!(window!());

  // let cookie = get_cookies!(html_document);
  // debug!("{cookie:#?}");

  let create_flow: Resource<Result<_, ory_kratos_client::apis::Error<_>>> = use_resource(
    move || async move { to_session(&Configuration::create(), None, None, None).await },
  );

  let mut state: bool = false;
  let mut timeout: Option<gloo_timers::callback::Timeout> = None;

  match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        // debug!("{res:?}");
        if let Some(active) = res.active {
          if active {
            state = true;
          }
        }
        if let Some(expires_at) = &res.expires_at {
          let timestamp = DateTime::parse_from_rfc3339(expires_at);
          match timestamp {
            Ok(dt) => {
              debug!("{dt:?}");
              let duration = dt.signed_duration_since(Utc::now()).num_milliseconds();
              let duration = duration.try_into().unwrap_or(0);
              // let _new_cookie = html_document.set_cookie(&format!(
              //   "local_session={duration}; SameSite=Strict; max-age={}; Secure",
              //   duration / 1000
              // ));

              if duration > 0 {
                debug!("Setting Session state to true");
                state = true;
                timeout = Some(Timeout::new(duration, move || {
                  *use_context::<Session>().state.write() = false;
                }));
              }
            }
            Err(err) => error!("{err:?}"),
          }
        }
      }
      Err(err) => {
        error!("{err:?}");
        state = false;
      }
    },
    None => {
      state = false;
    }
  };

  // navigator().replace(Route::Home {});

  // let cookie = get_cookies!(html_document);
  // debug!("{cookie:#?}");
  // rsx! {
  //   {
  //       *use_context::<Session>().state.write() = state;
  //       *use_context::<Session>().timeout.write() = timeout;
  //   }
  // }
  Ok((state, timeout))
}

#[component]
pub fn OrySession() -> Element {
  let create_flow: Resource<Result<_, ory_kratos_client::apis::Error<_>>> = use_resource(
    move || async move { to_session(&Configuration::create(), None, None, None).await },
  );

  let mut state: bool = false;
  let mut timeout: Option<gloo_timers::callback::Timeout> = None;

  match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        // debug!("{res:?}");
        if let Some(active) = res.active {
          if active {
            state = true;
          }
        }
        if let Some(expires_at) = &res.expires_at {
          let timestamp: Result<DateTime<chrono::FixedOffset>, chrono::ParseError> =
            DateTime::parse_from_rfc3339(expires_at);
          match timestamp {
            Ok(dt) => {
              debug!("{dt:?}");
              let duration = dt.signed_duration_since(Utc::now()).num_milliseconds();
              let duration = duration.try_into().unwrap_or(0);
              // let _new_cookie = html_document.set_cookie(&format!(
              //   "local_session={duration}; SameSite=Strict; max-age={}; Secure",
              //   duration / 1000
              // ));

              if duration > 0 {
                debug!("Setting Session state to true");
                state = true;
                timeout = Some(Timeout::new(duration, move || {
                  *use_context::<Session>().state.write() = false;
                }));
              }
            }
            Err(err) => error!("{err:?}"),
          }
        }
      }
      Err(err) => {
        error!("{err:?}");
        state = false;
      }
    },
    None => {
      state = false;
    }
  };

  // navigator().replace(Route::Home {});

  // let cookie = get_cookies!(html_document);
  // debug!("{cookie:#?}");

  *use_context::<Session>().state.write() = state;
  *use_context::<Session>().timeout.write() = timeout;

  rsx! {
    {format!("{:#?}", use_context::<Session>())}
  }
}
