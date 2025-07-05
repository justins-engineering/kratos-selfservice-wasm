use crate::KRATOS_BROWSER_URL;
use dioxus::prelude::*;

#[component]
pub fn Login(flow: String) -> Element {
  rsx! {
    div { class: "mx-auto w-full max-w-sm",
      div { class: "mt-10",
        form {
          "accept-charset": "UTF-8",
          action: "{KRATOS_BROWSER_URL}/self-service/login?flow={flow}",
          class: "",
          method: "post",
          input {
            autocomplete: "off",
            name: "csrf_token",
            r#type: "hidden",
            value: "{flow}",
                    // "68RsMTGRKXjtOFgQSgJdlqSak3Kf3y/HG37qG0XS26TDBB7mH4ILC/Yewuu7MV4q7gspVIN3JA4ds0ed65XIng=="
          // csrf_token_806060ca5bf70dff3caa0e5c860002aade9d470a5a4dce73bcfa7ba10778f481=6KWdHLVzpZMMWIsKvxPONxdMGqV4tOq7xQ6xpOrR9D0=
          }
          div { class: "mt-2",
            fieldset { class: "fieldset",
              legend { class: "fieldset-legend text-2xl", "Login" }

              label { class: "floating-label my-4",
                span { "Email" }
                input {
                  required: true,
                  autocomplete: "email",
                  autofocus: "autofocus",
                  class: "input validator w-full",
                  id: "user_login",
                  name: "identifier",
                  placeholder: "Email",
                  r#type: "email",
                }
                div { class: "validator-hint hidden", "Enter valid email address" }
              }

              label { class: "floating-label",
                span { "Password" }
                input {
                  required: true,
                  autocomplete: "current-password",
                  class: "input w-full",
                  id: "user_password",
                  name: "password",
                  placeholder: "Password",
                  r#type: "password",
                }
              }
              a {
                class: "leading-6 link-primary link-hover",
                href: "/users/password/new",
                "Forgot your password?"
              }

              input {
                class: "btn btn-primary w-full my-4",
                name: "method",
                r#type: "submit",
                value: "password",
                "Login"
              }
            }
          }
        }
        p { class: "text-sm leading-6",
          "Don't have an account? "
          a {
            class: "link-primary link-hover",
            href: "http://127.0.0.1:4433/self-service/registration/browser",
            "Get started →"
          }
        }
      }
    }
  }
}
