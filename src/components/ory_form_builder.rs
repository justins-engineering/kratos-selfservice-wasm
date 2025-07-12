use dioxus::prelude::*;
use ory_kratos_client::models::UiNodeAttributes::*;

#[component]
fn InputField(
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
        autocomplete: if let Some(a) = attrs.autocomplete { format!("{:?}", a).to_lowercase() },
        autofocus: "autofocus",
        class: "input w-full",
        class: if validate { "validator" },
        disabled: attrs.disabled,
        id: if let Some(ref label) = meta { format!("{}", label.id) },
        name: attrs.name,
        placeholder: if let Some(ref label) = meta { label.text.clone() } else { format!("{:?}", attrs.r#type) },
        r#type: format!("{:?}", attrs.r#type).to_lowercase(),
        pattern: if let Some(pattern) = pattern { pattern },
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
fn InputButton(
  meta: Option<Box<ory_kratos_client::models::UiText>>,
  attrs: Box<ory_kratos_client::models::UiNodeInputAttributes>,
) -> Element {
  rsx! {
    button {
      disabled: attrs.disabled,
      autofocus: "autofocus",
      class: "btn btn-primary w-full my-4",
      id: if let Some(ref label) = meta { format!("{}", label.id) },
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
pub fn FormBuilder(nodes: Vec<ory_kratos_client::models::UiNode>) -> Element {
  rsx! {
    for node in nodes {
      match *node.attributes {
          Input(i) => {
              match i.r#type {
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Text => {
                      rsx! {
                        InputField { meta: node.meta.label, attrs: i, validate: false }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Password => {
                      rsx! {
                        InputField {
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
                      todo!()
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Checkbox => {
                      todo!()
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Hidden => {
                      rsx! {
                        input {
                          autocomplete: if let Some(a) = i.autocomplete { format!("{:?}", a).to_lowercase() },
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
                        InputField {
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
                      todo!()
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Submit => {
                      rsx! {
                        InputButton { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Button => {
                      rsx! {
                        InputButton { meta: node.meta.label, attrs: i }
                      }
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::DatetimeLocal => {
                      todo!()
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Date => {
                      todo!()
                  }
                  ory_kratos_client::models::ui_node_input_attributes::TypeEnum::Url => {
                      todo!()
                  }
              }
          }
          Text(_text) => todo!(),
          Img(_img) => todo!(),
          A(_link) => todo!(),
          Script(_script) => todo!(),
      }
    }
  }
}
