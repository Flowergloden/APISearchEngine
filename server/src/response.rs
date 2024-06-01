use rocket::serde::{Deserialize, Serialize};

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
