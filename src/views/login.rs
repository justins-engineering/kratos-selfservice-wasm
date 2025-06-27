use dioxus::prelude::*;

#[component]
pub fn Login(flow: String) -> Element {
  rsx! {
    div { class: "mx-auto w-full max-w-sm",
      div { class: "mt-10",
        form {
          "accept-charset": "UTF-8",
          action: "/users/sign_in",
          class: "",
          method: "post",
          input {
            autocomplete: "off",
            name: "authenticity_token",
            r#type: "hidden",
            value: "{flow}",
          }
          div { class: "mt-2",
            fieldset { class: "fieldset",
              legend { class: "fieldset-legend text-2xl", "Login" }

              label {
                class: "floating-label my-4",
                span { "Email" }
                input {
                  required: true,
                  autocomplete: "email",
                  autofocus: "autofocus",
                  class: "input validator w-full",
                  id: "user_login",
                  name: "user[login]",
                  placeholder: "Email",
                  r#type: "email",
                }
                div { class: "validator-hint hidden", "Enter valid email address" }
              }

              label {
                class: "floating-label",
                span { "Password" }
                input {
                  required: true,
                  autocomplete: "current-password",
                  class: "input w-full",
                  id: "user_password",
                  name: "user[password]",
                  placeholder: "Password",
                  r#type: "password",
                }
              }
              a { class: "leading-6 link-primary link-hover", href: "/users/password/new", "Forgot your password?" }

              input {
                class: "btn btn-primary w-full my-4",
                "data-disable-with": "Log in",
                name: "commit",
                r#type: "submit",
                value: "Login",
              }
            }
          }
        }
        p { class: "text-sm leading-6",
          "Don't have an account? "
          a { class: "link-primary link-hover", href: "http://127.0.0.1:4433/self-service/registration/browser", "Get started →" }
        }
      }
    }
  }
}
