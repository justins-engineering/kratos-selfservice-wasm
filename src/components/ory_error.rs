use crate::Route;
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::{
  CreateBrowserLoginFlowError, CreateBrowserLogoutFlowError, CreateBrowserRecoveryFlowError,
  CreateBrowserRegistrationFlowError, ToSessionError,
};
use ory_kratos_client::apis::ResponseContent;
use ory_kratos_client::models::error_generic::ErrorGeneric;

fn error_content_rsx(err: ErrorGeneric) -> Element {
  rsx! {
    div { class: "text-center max-h-screen max-w-none",
      h1 { class: "text-2xl my-8", {err.error.message} }
      p { class: "font-light m-8", {err.error.reason} }
      Link { to: Route::Home {}, class: "btn btn-primary my-8", "Go Home" }
    }
  }
}

fn error_content_js(err: serde_json::Value) -> Element {
  rsx! {
    div { class: "text-center max-h-screen max-w-none",
      p { class: "font-light m-8", {err.as_str()} }
      Link { to: Route::Home {}, class: "btn btn-primary my-8", "Go Home" }
    }
  }
}

pub trait DisplayError {
  fn view_response_content(self) -> Element;
}

impl DisplayError for ResponseContent<CreateBrowserRegistrationFlowError> {
  fn view_response_content(self) -> Element {
    if let Some(ent) = self.entity {
      match ent {
        CreateBrowserRegistrationFlowError::DefaultResponse(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
        CreateBrowserRegistrationFlowError::UnknownValue(value) => rsx! {
          {error_content_js(value)}
        },
      }
    } else {
      rsx! {
        p { {self.content.to_string()} }
      }
    }
  }
}

impl DisplayError for ResponseContent<CreateBrowserLoginFlowError> {
  fn view_response_content(self) -> Element {
    if let Some(ent) = self.entity {
      match ent {
        CreateBrowserLoginFlowError::DefaultResponse(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
        CreateBrowserLoginFlowError::UnknownValue(value) => {
          rsx! {
            {error_content_js(value)}
          }
        }
        CreateBrowserLoginFlowError::Status400(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
      }
    } else {
      rsx! {
        p { {self.content.to_string()} }
      }
    }
  }
}

impl DisplayError for ResponseContent<CreateBrowserLogoutFlowError> {
  fn view_response_content(self) -> Element {
    if let Some(ent) = self.entity {
      match ent {
        CreateBrowserLogoutFlowError::UnknownValue(value) => {
          rsx! {
            {error_content_js(value)}
          }
        }
        CreateBrowserLogoutFlowError::Status400(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
        CreateBrowserLogoutFlowError::Status401(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
        CreateBrowserLogoutFlowError::Status500(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
      }
    } else {
      rsx! {
        p { {self.content.to_string()} }
      }
    }
  }
}

impl DisplayError for ResponseContent<ToSessionError> {
  fn view_response_content(self) -> Element {
    if let Some(ent) = self.entity {
      match ent {
        ToSessionError::DefaultResponse(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
        ToSessionError::UnknownValue(value) => {
          rsx! {
            {error_content_js(value)}
          }
        }
        ToSessionError::Status401(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
        ToSessionError::Status403(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
      }
    } else {
      rsx! {
        p { {self.content.to_string()} }
      }
    }
  }
}

impl DisplayError for ResponseContent<CreateBrowserRecoveryFlowError> {
  fn view_response_content(self) -> Element {
    if let Some(ent) = self.entity {
      match ent {
        CreateBrowserRecoveryFlowError::DefaultResponse(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
        CreateBrowserRecoveryFlowError::UnknownValue(value) => {
          rsx! {
            {error_content_js(value)}
          }
        }
        CreateBrowserRecoveryFlowError::Status400(error_generic) => rsx! {
          {error_content_rsx(error_generic)}
        },
      }
    } else {
      rsx! {
        p { {self.content.to_string()} }
      }
    }
  }
}
