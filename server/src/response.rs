use rocket::serde::{Deserialize, Serialize};

use crate::parser::Tag;

fn get_ref_link(filename: &str) -> String {
    format!("https://zh.cppreference.com/w/{filename}")
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Body {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Item {
    kind: ItemKind,
    path: String,
    source: String,
}

impl Item {
    pub fn mock(name: &str) -> Self {
        Self {
            kind: ItemKind::File,
            path: name.to_string(),
            source: name.to_string(),
        }
    }
}

impl From<Tag> for Item {
    fn from(value: Tag) -> Self {
        match value {
            Tag::File { name, filename, .. } => Self {
                kind: ItemKind::File,
                path: name,
                source: get_ref_link(&filename),
            },
            Tag::Namespace { name, filename, .. } => Self {
                kind: ItemKind::Namespace,
                path: name,
                source: get_ref_link(&filename),
            },
            Tag::Class { name, filename, .. } => Self {
                kind: ItemKind::Class,
                path: name,
                source: get_ref_link(&filename),
            },
            Tag::Function {
                name,
                af,
                of_class,
                of_namespace,
                ..
            } => {
                let path = if of_class.is_some() {
                    format!("{}::{}", of_class.unwrap(), name)
                } else {
                    format!("{}::{}", of_namespace.unwrap_or(String::new()), name)
                };
                Self {
                    kind: ItemKind::Function,
                    path,
                    source: get_ref_link(&af),
                }
            }
            Tag::Variable {
                name,
                af,
                of_class,
                of_namespace,
                ..
            } => {
                let path = if of_class.is_some() {
                    format!("{}::{}", of_class.unwrap(), name)
                } else {
                    format!("{}::{}", of_namespace.unwrap_or(String::new()), name)
                };
                Self {
                    kind: ItemKind::Variable,
                    path,
                    source: get_ref_link(&af),
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "lowercase")]
pub enum ItemKind {
    File,
    Namespace,
    Variable,
    Function,
    Class,
}
