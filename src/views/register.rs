use dioxus::prelude::*;

// const {
//       flow,
//       aal = "",
//       refresh = "",
//       return_to = "",
//       organization = "",
//       via = "",
//       login_challenge,
//     } = req.query

#[component]
pub fn Register(flow: String) -> Element {
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
              legend { class: "fieldset-legend text-2xl", "Register" }

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
                class: "floating-label my-4",
                span { "Password" }
                input {
                  required: true,
                  title: "Must be more than 8 characters, including number, lowercase letter, uppercase letter",
                  pattern: "(?=.*\\d)(?=.*[a-z])(?=.*[A-Z]).{{8,}}",
                  minlength: "8",
                  autocomplete: "current-password",
                  class: "input validator w-full",
                  id: "user_password",
                  name: "user[password]",
                  placeholder: "Password",
                  r#type: "password",
                }
                p {
                  class: "validator-hint hidden",
                  "Password must be more than 8 characters, and include:"
                  ul {
                    class: "list-disc list-inside",
                    li { "At least one number" }
                    li { "At least one lowercase letter" }
                    li { "At least one uppercase letter" }
                  }
                }
              }

              input {
                class: "btn btn-primary w-full my-4",
                "data-disable-with": "Log in",
                name: "commit",
                r#type: "submit",
                value: "Sign Up",
              }
            }
          }
        }
        p { class: "text-sm leading-6",
          "Already have an account? "
          a { class: "link-primary link-hover", href: "http://127.0.0.1:4433/self-service/login/browser", "Login →" }
        }
      }
    }
  }
}
