use crate::{Configuration, Create, Route, Session, SESSION_COOKIE_NAME};
use chrono::{DateTime, FixedOffset, Utc};
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

const COOKIE_STR_LEN: usize = SESSION_COOKIE_NAME.len()
  // + u32::MAX.to_string().len()
  + 10
  + "2025-08-05T17:14:07.837312011Z".len()
  + "=; path=/; SameSite=Strict; max-age=; Secure".len();

pub fn remove_session_cookie() {
  let html_document = html_document!(window!());

  let mut cookie_str = String::with_capacity(COOKIE_STR_LEN);
  cookie_str.push_str(SESSION_COOKIE_NAME);
  cookie_str.push_str("=0; path=/; SameSite=Strict; expires=Thu, 01 Jan 1970 00:00:00 UTC; Secure");

  let new_cookie = html_document.set_cookie(&cookie_str);

  match new_cookie {
    Ok(_) => {}
    Err(_) => {
      error!("Failed to set cookie");
    }
  }
}

pub fn set_session_cookie() -> bool {
  let html_document: web_sys::HtmlDocument = html_document!(window!());

  let create_flow: Resource<Result<_, ory_kratos_client::apis::Error<_>>> = use_resource(
    move || async move { to_session(&Configuration::create(), None, None, None).await },
  );

  if let Some(Ok(session)) = &*create_flow.read() {
    if let Some(expires_at) = &session.expires_at {
      let timestamp = DateTime::parse_from_rfc3339(expires_at);
      match timestamp {
        Ok(dt) => {
          let duration = dt.signed_duration_since(Utc::now()).num_milliseconds();
          let duration = duration.try_into().unwrap_or(0);

          let mut cookie_str = String::with_capacity(COOKIE_STR_LEN);
          cookie_str.push_str(SESSION_COOKIE_NAME);
          cookie_str.push('=');
          cookie_str.push_str(expires_at);
          cookie_str.push_str("; path=/; SameSite=Strict; max-age=");
          cookie_str.push_str(&(duration / 1000).to_string());
          cookie_str.push_str("; Secure");

          let new_cookie = html_document.set_cookie(&cookie_str);

          match new_cookie {
            Ok(_) => {
              // let timeout = Timeout::new(duration, move || {
              //   *use_context::<Session>().state.write() = false;
              //   remove_session_cookie();
              // });
              // timeout.forget();
              return true;
            }
            Err(_) => {
              error!("Failed to set cookie");
            }
          }
        }
        Err(err) => error!("{err:?}"),
      }
    }
  };
  debug!("{:?}", html_document.cookie());

  false
}

#[component]
pub fn SetSessionCookie(state: bool) -> Element {
  let html_document: web_sys::HtmlDocument = html_document!(window!());

  let create_flow: Resource<Result<_, ory_kratos_client::apis::Error<_>>> = use_resource(
    move || async move { to_session(&Configuration::create(), None, None, None).await },
  );

  if state {
    if let Some(Ok(session)) = &*create_flow.read() {
      if let Some(expires_at) = &session.expires_at {
        let timestamp = DateTime::parse_from_rfc3339(expires_at);
        match timestamp {
          Ok(dt) => {
            let duration = dt.signed_duration_since(Utc::now()).num_milliseconds();
            let duration = duration.try_into().unwrap_or(0);

            let mut cookie_str = String::with_capacity(COOKIE_STR_LEN);
            cookie_str.push_str(SESSION_COOKIE_NAME);
            cookie_str.push('=');
            cookie_str.push_str(expires_at);
            cookie_str.push_str("; path=/; SameSite=Strict; max-age=");
            cookie_str.push_str(&(duration / 1000).to_string());
            cookie_str.push_str("; Secure");

            let new_cookie = html_document.set_cookie(&cookie_str);

            match new_cookie {
              Ok(_) => {
                navigator().replace(Route::Home {});
              }
              Err(_) => {
                error!("Failed to set cookie");
              }
            }
          }
          Err(err) => error!("{err:?}"),
        }
      }
    };
  } else {
    remove_session_cookie();
    navigator().replace(Route::Home {});
  }
  debug!("{:?}", html_document.cookie());

  rsx!()
}

pub fn session_cookie_valid() -> bool {
  let html_document = html_document!(window!());
  let cookie_string = get_cookies!(html_document);
  debug!("{cookie_string:?}");
  let cookies = cookie_string.split(';');
  debug!("{cookies:?}");

  for cookie in cookies {
    if cookie.contains(SESSION_COOKIE_NAME) {
      debug!("contains name");
      let mut c = cookie_string.split('=');
      if let Some(expiry) = c.next_back() {
        debug!("{expiry:?}");
        let timestamp: Result<DateTime<FixedOffset>, chrono::ParseError> =
          DateTime::parse_from_rfc3339(expiry);
        match timestamp {
          Ok(dt) => {
            debug!("{dt:?}");
            let now = Utc::now().with_timezone(dt.offset());
            debug!("{now:?}");
            if now < dt {
              return true;
            }
          }
          Err(err) => error!("{err:?}"),
        }
      }
    }
  }

  false
}
