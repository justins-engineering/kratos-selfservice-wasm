use dioxus::logger::tracing::{info, Level};
use dioxus::prelude::*;

mod kratos;

pub use crate::kratos::status_check;
use ory_kratos_client::apis::configuration::Configuration;

const KRATOS_PUBLIC_URL: &str = "http://kratos:4433/";
const KRATOS_BROWSER_URL: &str = "http://127.0.0.1:4433/";
const COOKIE_SECRET: &str = "changeme";
const CSRF_COOKIE_NAME: &str = "ory_csrf_ui";
const CSRF_COOKIE_SECRET: &str = "changeme";

// const ORY_CONFIG: Configuration = Configuration {
//   base_path: "http://localhost:8080".to_owned(),
//   user_agent: Some("OpenAPI-Generator/v1.3.8/rust".to_owned()),
//   client: reqwest::Client::new(),
//   basic_auth: None,
//   oauth_access_token: None,
//   bearer_access_token: None,
//   api_key: None,
// };

trait Create {
  fn create() -> Configuration;
}

impl Create for Configuration {
  fn create() -> Configuration {
    Configuration {
      base_path: "http://localhost:8080".to_owned(),
      user_agent: Some("OpenAPI-Generator/v1.3.8/rust".to_owned()),
      client: reqwest::Client::new(),
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
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
  dioxus::logger::init(Level::INFO).expect("logger failed to init");
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  info!("App rendered");
  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    Router::<Route> {}
  }
}

/// Home page
#[component]
fn Home() -> Element {
  let config = Configuration {
    base_path: KRATOS_BROWSER_URL.to_string(),
    user_agent: Some("OpenAPI-Generator/v1.3.8/rust".to_owned()),
    client: reqwest::Client::new(),
    basic_auth: None,
    oauth_access_token: None,
    bearer_access_token: None,
    api_key: None,
  };
  let version = use_resource(move || {
    let value = config.clone();
    async move { status_check(&value.clone()).await }
  });

  rsx! {
    article { class: "prose max-w-none",
      h1 { "Welcome to the Ory Account Experience!" }
      p {
        "Let your customers sign up, log in and manage their account using Ory's standard experience. Here you can preview, test and learn to integrate it into your application."
      }
      p { "Your Ory Account Experience is running at 127.0.0.1:4455." }
      code { "Kratos SDK Version: {version:?}" }
      hr {}
      h2 { "Core concepts" }
      p { "Here are some useful documentation pieces that help you get started right away." }
    }
    Cards {}
  }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
  rsx! {
    div { id: "blog",

      // Content
      h1 { "This is blog #{id}!" }
      p {
        "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
      }

      // Navigation links
      Link { to: Route::Blog { id: id - 1 }, "Previous" }
      span { " <---> " }
      Link { to: Route::Blog { id: id + 1 }, "Next" }
    }
  }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
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
            Link { to: Route::Blog { id: 1 }, "Blog" }
          }
          li {
            a { href: "http://127.0.0.1:4455/welcome", "Overview" }
          }
          li {
            a { href: "http://127.0.0.1:4455/sessions", "Session Information" }
          }
          li {
            h2 { class: "menu-title text-neutral", "Default User Interfaces" }
            ul {
              li {
                a { href: "http://127.0.0.1:4455/login", "Sign In" }
              }
              li {
                a { href: "http://127.0.0.1:4455/registration", "Sign Up" }
              }
              li {
                a { href: "http://127.0.0.1:4455/recovery", "Account Recovery" }
              }
              li {
                a { href: "http://127.0.0.1:4455/verification",
                  "Account Verification"
                }
              }
              li {
                a { href: "http://127.0.0.1:4455/settings", "Account Settings" }
              }
              li {
                a { href: "http://127.0.0.1:4433/self-service/logout?token=",
                  "Log out"
                }
              }
            }
          }
        }
      }
    }
  }
}

#[component]
pub fn Cards() -> Element {
  rsx! {
    div { class: "lg:flex lg:flex-row lg:justify-between",
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content lg:h-full my-4 lg:basis-1/6",
        a { href: "https://www.ory.sh/docs/getting-started/integrate-auth/expressjs",
          div { class: "card-body",
            h2 { class: "card-title", "Getting Started" }
            p {
              "Jump start your project and complete the quickstart tutorial to get a broader overview of Ory Network."
            }
          }
        }
      }
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content lg:h-full my-4 lg:basis-1/6",
        a { href: "https://www.ory.sh/docs/kratos/self-service",
          div { class: "card-body",
            h2 { class: "card-title", "User flows" }
            p {
              "Implement flows that users perform themselves as opposed to administrative intervention."
            }
          }
        }
      }
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content lg:h-full my-4 lg:basis-1/6",
        a { href: "https://www.ory.sh/docs/kratos/manage-identities/identity-schema",
          div { class: "card-body",
            h2 { class: "card-title", "Identities 101" }
            p {
              "Every identity can have its own model - get to know the ins and outs of Identity Schemas."
            }
          }
        }
      }
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content lg:h-full my-4 lg:basis-1/6",
        a { href: "https://www.ory.sh/docs/kratos/session-management/overview",
          div { class: "card-body",
            h2 { class: "card-title", "Sessions" }
            p { "Ory Network manages sessions for you - get to know how sessions work." }
          }
        }
      }
      div { class: "card bg-primary hover:bg-primary/90 text-primary-content lg:h-full my-4 lg:basis-1/6",
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
