mod components;
mod views;

use crate::components::{
  OryLogOut, SetSessionCookie, remove_session_cookie, session_cookie_valid, set_session_cookie,
};
use dioxus::logger::tracing::{debug, error};
use dioxus::prelude::*;
use ory_kratos_client_wasm::apis::configuration::Configuration;

use crate::views::{
  AccountRecovery, LoginFlow, PageNotFound, RecoveryFlow, RegisterFlow, ServerError, SessionInfo,
  Settings, SettingsFlow, SignIn, SignUp, VerificationFlow, Verify,
};

#[cfg(feature = "web")]
use gloo_timers::callback::Timeout;

const KRATOS_BROWSER_URL: &str = "http://127.0.0.1:4433";
const SESSION_COOKIE_NAME: &str = "session_expiry";

#[derive(Clone, Copy, Debug)]
struct Session {
  state: Signal<bool>,
}

trait Create {
  fn create() -> Configuration;
}

impl Create for Configuration {
  fn create() -> Configuration {
    Configuration {
      base_path: KRATOS_BROWSER_URL.to_owned(),
      user_agent: None,
      basic_auth: None,
      oauth_access_token: None,
      bearer_access_token: None,
      api_key: None,
    }
  }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
      #[route("/")]
      Home {},
      #[route("/session")]
      SessionInfo {},
      #[route("/sign-in")]
      SignIn {},
      #[route("/login?:flow")]
      LoginFlow { flow: String },
      #[route("/sign-up")]
      SignUp {},
      #[route("/registration?:flow")]
      RegisterFlow { flow: String },
      #[route("/verify")]
      Verify {},
      #[route("/verification?:flow")]
      VerificationFlow { flow: String },
      #[route("/my-settings")]
      Settings {},
      #[route("/settings?:flow")]
      SettingsFlow { flow: String },
      #[route("/account-recovery")]
      AccountRecovery {},
      #[route("/recovery?:flow")]
      RecoveryFlow { flow: String },
      #[route("/session/local?:state")]
      SetSessionCookie { state: bool },
    #[end_layout]
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/error?:id")]
    ServerError { id: String },
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
  dioxus::logger::initialize_default();
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  use_context_provider(|| Session {
    state: Signal::new(false),
  });

  let set_state = use_resource(move || async move { session_cookie_valid().await });
  (set_state)();

  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    Router::<Route> {}
  }
}

/// Home page
#[component]
fn Home() -> Element {
  rsx! {
    article { class: "max-w-none",
      h1 { class: "mb-8 text-4xl font-extrabold", "Welcome to the Ory Account Experience!" }
      p { class: "my-5 text-base-content opacity-80",
        "Let your customers sign up, log in and manage their account using Ory's standard experience. Here you can preview, test and learn to integrate it into your application."
      }
      hr { class: "my-12 border-base-content" }
      h2 { class: "my-4 text-2xl font-bold", "Core concepts" }
      p { "Here are some useful documentation pieces that help you get started right away." }
    }
    Cards {}
  }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
  debug!("{:?}", use_context::<Session>());

  let links = if *use_context::<Session>().state.read() {
    rsx! {
      li { class: "menu-disabled",
        Link { to: Route::SignIn {}, "Sign In" }
      }
      li { class: "menu-disabled",
        Link { to: Route::SignUp {}, "Sign Up" }
      }
      li { class: "menu-disabled",
        Link { to: Route::AccountRecovery {}, "Account Recovery" }
      }
      li {
        Link { to: Route::Verify {}, "Account Verification" }
      }
      li {
        Link { to: Route::Settings {}, "Account Settings" }
      }
      OryLogOut {}
    }
  } else {
    rsx! {
      li {
        Link { to: Route::SignIn {}, "Sign In" }
      }
      li {
        Link { to: Route::SignUp {}, "Sign Up" }
      }
      li {
        Link { to: Route::AccountRecovery {}, "Account Recovery" }
      }
      li {
        Link { to: Route::Verify {}, "Account Verification" }
      }
      li { class: "menu-disabled",
        Link { to: Route::Settings {}, "Account Settings" }
      }
      li { class: "menu-disabled",
        a { href: "", "Log out" }
      }
    }
  };

  rsx! {
    div { class: "drawer lg:drawer-open",
      input {
        class: "drawer-toggle",
        id: "nav-drawer",
        r#type: "checkbox",
      }
      div { class: "drawer-content flex flex-col max-w-none p-4 lg:p-12",
        span { class: "py-4",
          label {
            class: "btn btn-primary drawer-button lg:hidden py-2",
            r#for: "nav-drawer",
            "Menu"
          }
        }
        Outlet::<Route> {}
      }
      div { class: "drawer-side",
        label {
          aria_label: "close sidebar",
          class: "drawer-overlay",
          r#for: "nav-drawer",
        }
        ul { class: "menu menu-lg bg-base-200 text-base-content min-h-full w-80 p-4",
          li { class: "menu-title text-primary", "Welcome to Ory" }
          li {
            Link { to: Route::Home {}, "Home" }
          }
          li {
            Link { to: Route::SessionInfo {}, "Session Information" }
          }
          li {
            h2 { class: "menu-title", "Default User Interfaces" }
            ul { {links} }
          }
        }
      }
    }
  }
}

#[component]
pub fn Cards() -> Element {
  rsx! {
    div { class: "xl:flex xl:flex-row xl:justify-between",
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content xl:h-full my-4 xl:basis-1/6",
        a { href: "https://www.ory.sh/docs/getting-started/integrate-auth/expressjs",
          div { class: "card-body",
            h2 { class: "card-title", "Getting Started" }
            p {
              "Jump start your project and complete the quickstart tutorial to get a broader overview of Ory Network."
            }
          }
        }
      }
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content xl:h-full my-4 xl:basis-1/6",
        a { href: "https://www.ory.sh/docs/kratos/self-service",
          div { class: "card-body",
            h2 { class: "card-title", "User flows" }
            p {
              "Implement flows that users perform themselves as opposed to administrative intervention."
            }
          }
        }
      }
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content xl:h-full my-4 xl:basis-1/6",
        a { href: "https://www.ory.sh/docs/kratos/manage-identities/identity-schema",
          div { class: "card-body",
            h2 { class: "card-title", "Identities 101" }
            p {
              "Every identity can have its own model - get to know the ins and outs of Identity Schemas."
            }
          }
        }
      }
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content xl:h-full my-4 xl:basis-1/6",
        a { href: "https://www.ory.sh/docs/kratos/session-management/overview",
          div { class: "card-body",
            h2 { class: "card-title", "Sessions" }
            p { "Ory Network manages sessions for you - get to know how sessions work." }
          }
        }
      }
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content xl:h-full my-4 xl:basis-1/6",
        a { href: "https://www.ory.sh/docs/kratos/bring-your-own-ui/configure-ory-to-use-your-ui",
          div { class: "card-body",
            h2 { class: "card-title", "Custom UI" }
            p {
              "Implementing these pages in your language and framework of choice is straightforward using our SDKs."
            }
          }
        }
      }
    }
  }
}
