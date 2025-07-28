use crate::{Configuration, Create, Route, Session, KRATOS_LIFETIME};
use dioxus::logger::tracing::{debug, error};
use dioxus::prelude::*;
use ory_kratos_client::apis::frontend_api::{update_login_flow, update_registration_flow};
use ory_kratos_client::models::UiNodeAttributes::{Img, Input, Script, Text, A};
use ory_kratos_client::models::{
  UpdateLoginFlowBody, UpdateLoginFlowWithPasswordMethod, UpdateLoginFlowWithTotpMethod,
  UpdateRegistrationFlowBody, UpdateRegistrationFlowWithPasswordMethod,
};

#[cfg(feature = "web")]
use gloo_timers::callback::Timeout;

#[component]
fn InputFieldNode(
  meta: Option<Box<ory_kratos_client::models::UiText>>,
  attrs: Box<ory_kratos_client::models::UiNodeInputAttributes>,
  validate: bool,
  pattern: Option<String>,
  hint: Option<Element>,
) -> Element {
  rsx! {
    label { class: "floating-label my-4",
      span {
        {
            if let Some(ref label) = meta {
                label.text.clone()
            } else {
                format!("{:?}", attrs.r#type)
            }
        }
      }
      input {
        required: if let Some(r) = attrs.required { r },
        autocomplete: if let Some(a) = attrs.autocomplete { format!("{a:?}").to_lowercase() },
        class: "input w-full",
        class: if validate { "validator" },
        disabled: attrs.disabled,
        id: if let Some(ref label) = meta { label.id.to_string() },
        name: attrs.name,
        placeholder: if let Some(ref label) = meta { label.text.clone() } else { format!("{:?}", attrs.r#type) },
        r#type: format!("{:?}", attrs.r#type).to_lowercase(),
        pattern: if let Some(pattern) = pattern { pattern },
        value: if let Some(v) = attrs.value { if let Some(serde_json::Value::String(s)) = v { s } else { "".to_string() } },
      }
      if validate {
        div { class: "validator-hint hidden",
          if let Some(hint) = hint {
            {hint}
          }
        }
      }
    }
  }
}

#[component]
fn InputButtonNode(
  meta: Option<Box<ory_kratos_client::models::UiText>>,
  attrs: Box<ory_kratos_client::models::UiNodeInputAttributes>,
) -> Element {
  rsx! {
    button {
      disabled: attrs.disabled,
      class: "btn btn-primary w-full my-4",
      id: if let Some(ref label) = meta { label.id.to_string() },
      name: attrs.name,
      r#type: format!("{:?}", attrs.r#type).to_lowercase(),

      value: if let Some(v) = attrs.value { if let Some(t) = v {
          match t {
              serde_json::Value::String(s) => s,
              _ => "".to_string(),
          }
      } else {
          "".to_string()
      } },

      if let Some(ref label) = meta {
        {label.text.to_string()}
      }
    }
  }
}

#[component]
fn InputOtherNode(
  meta: Option<Box<ory_kratos_client::models::UiText>>,
  attrs: Box<ory_kratos_client::models::UiNodeInputAttributes>,
) -> Element {
  rsx! {
    if let Some(ref label) = meta {
      label { id: label.id, class: "w-full",
        {label.text.to_owned()}
        input {
          disabled: attrs.disabled,
          class: "input w-full",
          name: attrs.name,
          r#type: format!("{:?}", attrs.r#type).to_lowercase(),
          value: if let Some(v) = attrs.value { if let Some(t) = v {
              match t {
                  serde_json::Value::String(s) => s,
                  serde_json::Value::Number(n) => n.to_string(),
                  serde_json::Value::Bool(b) => b.to_string(),
                  serde_json::Value::Array(a) => format!("{a:?}"),
                  _ => "".to_string(),
              }
          } else {
              "".to_string()
          } },
        }
      }
    } else {
      input {
        disabled: attrs.disabled,
        class: "input w-full",
        name: attrs.name,
        r#type: format!("{:?}", attrs.r#type).to_lowercase(),

        value: if let Some(v) = attrs.value { if let Some(t) = v {
            match t {
                serde_json::Value::String(s) => s,
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Array(a) => format!("{a:?}"),
                _ => "".to_string(),
            }
        } else {
            "".to_string()
        } },
      }
    }
  }
}

#[component]
fn InputCheckBoxNode(
  meta: Option<Box<ory_kratos_client::models::UiText>>,
  attrs: Box<ory_kratos_client::models::UiNodeInputAttributes>,
) -> Element {
  rsx! {
    if let Some(ref label) = meta {
      label { id: label.id,
        {label.text.to_owned()}
        input {
          disabled: attrs.disabled,
          class: "input w-full",
          name: attrs.name,
          r#type: format!("{:?}", attrs.r#type).to_lowercase(),
          checked: if let Some(v) = attrs.value { if let Some(serde_json::Value::Bool(b)) = v { b } else { false } },
        }
      }
    } else {
      input {
        disabled: attrs.disabled,
        class: "input",
        name: attrs.name,
        r#type: format!("{:?}", attrs.r#type).to_lowercase(),
        checked: if let Some(v) = attrs.value { if let Some(serde_json::Value::Bool(b)) = v { b } else { false } },
      }
    }
  }
}

#[component]
fn ImageNode(
  meta: Option<Box<ory_kratos_client::models::UiText>>,
  attrs: Box<ory_kratos_client::models::UiNodeImageAttributes>,
) -> Element {
  rsx! {
    if let Some(ref label) = meta {
      label { id: label.id, class: "text-lg mb-4",
        {label.text.clone()}
        img {
          height: attrs.height,
          id: attrs.id,
          src: attrs.src,
          width: attrs.width,
          alt: label.text.to_owned(),
        }
      }
    } else {
      img {
        height: attrs.height,
        id: attrs.id,
        src: attrs.src,
        width: attrs.width,
      }
    }
  }
}

#[component]
fn TextNode(
  meta: Option<Box<ory_kratos_client::models::UiText>>,
  attrs: Box<ory_kratos_client::models::UiNodeTextAttributes>,
) -> Element {
  rsx! {
    if let Some(ref label) = meta {
      label { r#for: attrs.id.clone(), id: label.id, class: "text-lg",
        {label.text.to_owned()}
      }
    }
    p { id: attrs.id, class: "", {attrs.text.text} }
  }
}

#[component]
fn LinkNode(
  meta: Option<Box<ory_kratos_client::models::UiText>>,
  attrs: Box<ory_kratos_client::models::UiNodeAnchorAttributes>,
) -> Element {
  rsx! {
    if let Some(ref label) = meta {
      label { r#for: attrs.id.clone(), id: label.id, class: "text-lg",
        {label.text.to_owned()}
      }
    }
    a {
      id: attrs.id,
      class: "link-primary link-hover",
      href: attrs.href,
      {attrs.title.text}
    }
  }
}

#[component]
fn ScriptNode(attrs: Box<ory_kratos_client::models::UiNodeScriptAttributes>) -> Element {
  rsx! {
    script {
      r#async: attrs.r#async,
      crossorigin: attrs.crossorigin,
      id: attrs.id,
      integrity: attrs.integrity,
      nonce: attrs.nonce,
      referrerpolicy: attrs.referrerpolicy,
      src: attrs.src,
      r#type: attrs.r#type,
    }
  }
}

#[component]
fn MessageNode(message: ory_kratos_client::models::UiText) -> Element {
  rsx! {
    div {
      id: message.id,
      role: "alert",
      class: {
          match message.r#type {
              ory_kratos_client::models::ui_text::TypeEnum::Error => "alert alert-error",
              ory_kratos_client::models::ui_text::TypeEnum::Info => "alert alert-info",
              ory_kratos_client::models::ui_text::TypeEnum::Success => {
                  "alert alert-success"
              }
          }
      },
      span { {message.text} }
    }
  }
}

#[component]
fn NodeBuilder(nodes: Vec<ory_kratos_client::models::UiNode>) -> Element {
  rsx! {
    for node in nodes {
      match *node.attributes {
          Input(i) => {
              match i.r#type {
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Text => {
                      rsx! {
                        InputFieldNode { meta: node.meta.label, attrs: i, validate: false }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Password => {
                      rsx! {
                        InputFieldNode {
                          meta: node.meta.label,
                          attrs: i,
                          validate: true,
                          hint: rsx! {
                            "Password must be more than 8 characters, and include:"
                            ul { class: "list-disc list-inside",
                              li { "At least one number" }
                              li { "At least one lowercase letter" }
                              li { "At least one uppercase letter" }
                            }
                          },
                          pattern: "(?=.*\\d)(?=.*[a-z])(?=.*[A-Z]).{{8,}}",
                        // title: "Must be more than 8 characters, including number, lowercase letter, uppercase letter"
                        }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Number => {
                      rsx! {
                        InputOtherNode { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Checkbox => {
                      rsx! {
                        InputOtherNode { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Hidden => {
                      rsx! {
                        input {
                          autocomplete: if let Some(a) = i.autocomplete { format!("{a:?}").to_lowercase() },
                          disabled: i.disabled,
                          name: i.name,
                          id: if let Some(ref label) = node.meta.label { format!("{}", label.id) },
                          r#type: format!("{:?}", i.r#type).to_lowercase(),
                          value: if let Some(v) = i.value { if let Some(t) = v {
                              match t {
                                  serde_json::Value::String(s) => s,
                                  _ => "".to_string(),
                              }
                          } else {
                              "".to_string()
                          } },
                        }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Email => {
                      rsx! {
                        InputFieldNode {
                          meta: node.meta.label,
                          attrs: i,
                          validate: true,
                          hint: rsx! {
                            p { "Enter valid email address" }
                          },
                        }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Tel => {
                      rsx! {
                        InputOtherNode { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Submit => {
                      rsx! {
                        InputButtonNode { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Button => {
                      rsx! {
                        InputButtonNode { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::DatetimeLocal => {
                      rsx! {
                        InputOtherNode { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Date => {
                      rsx! {
                        InputOtherNode { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Url => {
                      rsx! {
                        InputOtherNode { meta: node.meta.label, attrs: i }
                      }
                  }
              }
          }
          Text(text) => {
              rsx! {
                TextNode { meta: node.meta.label, attrs: text }
              }
          }
          Img(img) => {
              rsx! {
                ImageNode { meta: node.meta.label, attrs: img }
              }
          }
          A(link) => {
              rsx! {
                LinkNode { meta: node.meta.label, attrs: link }
              }
          }
          Script(script) => {
              rsx! {
                ScriptNode { attrs: script }
              }
          }
      }
    }
  }
}

// #[derive(Clone, Debug, PartialEq)]
// pub enum AjaxFormType {
//   Login(UpdateLoginFlowBody),
//   Register(UpdateRegistrationFlowBody),
// }

// #[derive(Clone, Debug, PartialEq, Props)]
// pub struct AjaxFormBody<T: 'static + std::cmp::PartialEq> {
//   variant: T,
//   flow: String,
//   ui: ory_kratos_client::models::UiContainer,
// }

async fn login_submit_callback(flow: String, body: UpdateLoginFlowBody) {
  let update_flow = use_resource(move || {
    let id = flow.clone();
    let value = body.clone();
    async move { update_login_flow(&Configuration::create(), &id, value, None, None).await }
  });

  match &*update_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        debug!("{res:#?}");
        let mut state = use_context::<Session>().state;
        debug!("Setting Session state to true");
        *state.write() = true;
        let timeout = Timeout::new(KRATOS_LIFETIME, move || {
          debug!("Setting Session state to false");
          *state.write() = false;
        });
        timeout.forget();
        // navigator().replace(Route::Home {});
      }
      Err(err) => {
        error!("{err:#?}");
        // navigator().replace(Route::SignIn {});
      }
    },
    None => {
      error!("Update flow None!");
      // navigator().replace(Route::SignIn {});
    }
  };
}

pub trait AjaxFormBuilder {
  fn build_form(self, flow: String, ui: ory_kratos_client::models::UiContainer) -> Element;
}

impl AjaxFormBuilder for UpdateLoginFlowBody {
  fn build_form(self, flow: String, ui: ory_kratos_client::models::UiContainer) -> Element {
    // let get_flow = use_resource(move || async move {
    //   data.identifier = evt.values()["identifier"].as_value();
    //   data.password = evt.values()["password"].as_value();
    //   login_submit_callback(id.clone(), UpdateLoginFlowBody::Password(data)).await;
    // });

    rsx! {
      if let Some(messages) = ui.messages {
        for message in messages {
          MessageNode { message }
        }
      }
      form {
        action: ui.action.clone(),
        method: ui.method.clone(),
        onsubmit: match self {
            UpdateLoginFlowBody::Password(password) => {
                move |evt: FormEvent| {
                    evt.stop_propagation();
                    let mut data = password.clone();
                    let id = flow.clone();
                    {
                        async move {
                            data.identifier = evt.values()["identifier"].as_value();
                            data.password = evt.values()["password"].as_value();
                            login_submit_callback(
                                    id.clone(),
                                    UpdateLoginFlowBody::Password(data),
                                )
                                .await;
                        }
                    }
                }
            }
            UpdateLoginFlowBody::Oidc(oidc) => todo!(),
            UpdateLoginFlowBody::Totp(totp) => todo!(),
            UpdateLoginFlowBody::Webauthn(webauthn) => todo!(),
            UpdateLoginFlowBody::LookupSecret(lookup_secret) => todo!(),
            UpdateLoginFlowBody::Code(code) => todo!(),
            UpdateLoginFlowBody::Passkey(passkey) => todo!(),
            UpdateLoginFlowBody::IdentifierFirst(identifier_first) => todo!(),
        },
        div { class: "mt-2",
          fieldset { class: "fieldset",
            NodeBuilder { nodes: ui.nodes }
          }
        }
      }
    }
  }
}

#[component]
fn FormBuilder(
  body: UpdateLoginFlowBody,
  flow: String,
  ui: ory_kratos_client::models::UiContainer,
) -> Element {
  // let get_flow = use_resource(move || async move {
  //   data.identifier = evt.values()["identifier"].as_value();
  //   data.password = evt.values()["password"].as_value();
  //   login_submit_callback(id.clone(), UpdateLoginFlowBody::Password(data)).await;
  // });

  rsx! {
    if let Some(messages) = ui.messages {
      for message in messages {
        MessageNode { message }
      }
    }
    form {
      action: ui.action.clone(),
      method: ui.method.clone(),
      onsubmit: match body {
          UpdateLoginFlowBody::Password(password) => {
              move |evt: FormEvent| {
                  let mut data = password.clone();
                  let id = flow.clone();
                  {
                      async move {
                          data.identifier = evt.values()["identifier"].as_value();
                          data.password = evt.values()["password"].as_value();
                          login_submit_callback(
                                  id.clone(),
                                  UpdateLoginFlowBody::Password(data),
                              )
                              .await;
                      }
                  }
              }
          }
          UpdateLoginFlowBody::Oidc(oidc) => todo!(),
          UpdateLoginFlowBody::Totp(totp) => todo!(),
          UpdateLoginFlowBody::Webauthn(webauthn) => todo!(),
          UpdateLoginFlowBody::LookupSecret(lookup_secret) => todo!(),
          UpdateLoginFlowBody::Code(code) => todo!(),
          UpdateLoginFlowBody::Passkey(passkey) => todo!(),
          UpdateLoginFlowBody::IdentifierFirst(identifier_first) => todo!(),
      },
      div { class: "mt-2",
        fieldset { class: "fieldset",
          NodeBuilder { nodes: ui.nodes }
        }
      }
    }
  }
}

// impl AjaxFormBuilder for UpdateLoginFlowWithPasswordMethod {
//   fn build_form(self, flow: String, ui: ory_kratos_client::models::UiContainer) -> Element {
//     rsx! {
//       if let Some(messages) = ui.messages {
//         for message in messages {
//           MessageNode { message }
//         }
//       }
//       form {
//         action: ui.action.clone(),
//         method: ui.method.clone(),
//         onsubmit: move |evt: FormEvent| {
//             let mut data = self.clone();
//             let id = flow.clone();
//             {
//                 async move {
//                     data.identifier = evt.values()["identifier"].as_value();
//                     data.password = evt.values()["password"].as_value();
//                     login_submit_callback(
//                             id.clone(),
//                             UpdateLoginFlowBody::Password(Box::new(data)),
//                         )
//                         .await;
//                 }
//             }
//         },
//         div { class: "mt-2",
//           fieldset { class: "fieldset",
//             NodeBuilder { nodes: ui.nodes }
//           }
//         }
//       }
//     }
//   }
// }

// if node_groups.is_empty() {
//   rsx! {
//     if let Some(messages) = ui.messages {
//       for message in messages {
//         {message_node(message)}
//       }
//     }
//     form {
//       action: ui.action.clone(),
//       method: ui.method.clone(),
//       onsubmit: match ajax {
//           AjaxFormType::Login(login) => {
//               match login.clone() {
//                   UpdateLoginFlowBody::Password(mut password) => {
//                       move |evt: FormEvent| {
//                           password.identifier = evt.values()["identifier"].as_value();
//                           password.password = evt.values()["password"].as_value();
//                           debug!("{password:#?}",);
//                           login_submit_callback(flow.to_owned(), login.to_owned());
//                       }
//                   }
//                   UpdateLoginFlowBody::Oidc(oidc) => todo!(),
//                   UpdateLoginFlowBody::Totp(totp) => todo!(),
//                   UpdateLoginFlowBody::Webauthn(webauthn) => todo!(),
//                   UpdateLoginFlowBody::LookupSecret(lookup_secret) => todo!(),
//                   UpdateLoginFlowBody::Code(code) => todo!(),
//                   UpdateLoginFlowBody::Passkey(passkey) => todo!(),
//                   UpdateLoginFlowBody::IdentifierFirst(identifier_first) => todo!(),
//               }
//           }
//           AjaxFormType::Register(_) => todo!(),
//       },
//       div { class: "mt-2",
//         fieldset { class: "fieldset", {node_builder(default)} }
//       }
//     }
//   }
// } else {
//   let flow = flow.clone();
//   rsx! {
//     if let Some(messages) = ui.messages {
//       for message in messages {
//         {message_node(message)}
//       }
//     }
//     for node_group in node_groups {
//       form {
//         action: ui.action.clone(),
//         method: ui.method.clone(),
//         onsubmit: match ajax.clone() {
//             AjaxFormType::Login(login) => {
//                 match login.clone() {
//                     UpdateLoginFlowBody::Password(mut password) => {
//                         move |evt: FormEvent| {
//                             password.identifier = evt.values()["identifier"].as_value();
//                             password.password = evt.values()["password"].as_value();
//                             debug!("{password:#?}",);
//                             login_submit_callback(flow.to_owned(), login.to_owned());
//                         }
//                     }
//                     UpdateLoginFlowBody::Oidc(oidc) => todo!(),
//                     UpdateLoginFlowBody::Totp(totp) => todo!(),
//                     UpdateLoginFlowBody::Webauthn(webauthn) => todo!(),
//                     UpdateLoginFlowBody::LookupSecret(lookup_secret) => todo!(),
//                     UpdateLoginFlowBody::Code(code) => todo!(),
//                     UpdateLoginFlowBody::Passkey(passkey) => todo!(),
//                     UpdateLoginFlowBody::IdentifierFirst(identifier_first) => todo!(),
//                 }
//             }
//             AjaxFormType::Register(_) => todo!(),
//         },
//         div { class: "mt-2",
//           fieldset { class: "fieldset",
//             legend { class: "fieldset-legend text-xl",
//               {
//                   match node_group[0].group {
//                       ory_kratos_client::models::ui_node::GroupEnum::Password => "Password",
//                       ory_kratos_client::models::ui_node::GroupEnum::Oidc => "OIDC",
//                       ory_kratos_client::models::ui_node::GroupEnum::Profile => "Profile",
//                       ory_kratos_client::models::ui_node::GroupEnum::Code => "Code",
//                       ory_kratos_client::models::ui_node::GroupEnum::Totp => "TOTP",
//                       ory_kratos_client::models::ui_node::GroupEnum::LookupSecret => "Recovery",
//                       ory_kratos_client::models::ui_node::GroupEnum::Webauthn => {
//                           "Web Authentication"
//                       }
//                       ory_kratos_client::models::ui_node::GroupEnum::Passkey => "Passkey",
//                       ory_kratos_client::models::ui_node::GroupEnum::Captcha => "Captcha",
//                       ory_kratos_client::models::ui_node::GroupEnum::Saml => "SAML",
//                       _ => "",
//                   }
//               }
//             }
//             {node_builder(default.clone())}
//             {node_builder(node_group)}
//           }
//         }
//       }
//     }
//   }
// }
