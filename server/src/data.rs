use derive_builder::Builder;

pub trait TagData {}

#[derive(Debug, Builder)]
pub struct File {
    name: String,
    filename: String,
    namespace: String,
}