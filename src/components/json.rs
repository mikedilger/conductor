use dioxus::prelude::*;
use serde_json::Value;

const JSON_CSS: Asset = asset!("/assets/styling/json.css");

#[component]
pub fn Json(property: Value) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: JSON_CSS}

        div {
            class: "json",
            JsonInner {
                property: property,
                depth: 0
            }
        }
    }
}

#[component]
pub fn JsonInner(property: Value, depth: usize) -> Element {
    rsx! {
        match property {
            Value::String(s) => rsx!(span { "{s}" }),
            Value::Number(n) => rsx!(span { "{n}" }),
            Value::Bool(b) => rsx!(span { "{b}" }),
            Value::Array(arr) => rsx! {
                span { "[" }
                ul {
                    for item in arr.iter() {
                        li {
                            JsonInner {
                                property: item.clone(),
                                depth: depth + 2
                            }
                        }
                    }
                },
                span {
                    "]"
                }
            },
            Value::Object(obj) => rsx! {
                span { "{{" },
                ul {
                    for (key, value) in obj.iter() {
                        li {
                            span {
                                "{key}: "
                            },
                            JsonInner {
                                property: value.clone(),
                                depth: depth + 2
                            },
                        }
                    },
                }
                span {
                    "}}"
                },
            },
            _ => rsx!(span { "Unknown" }),
        }
    }
}
