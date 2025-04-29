use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MemeTplTextBox {
    #[serde(rename="w")]
    pub width: f32,
    #[serde(rename="h")]
    pub height: f32,
    #[serde(rename="t")]
    pub top: f32,
    #[serde(rename="l")]
    pub left: f32,
    #[serde(rename="r")]
    pub rotation: Option<i32>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MemeTpl {
    pub id: u32,
    #[serde(rename="w")]
    pub width: u32,
    #[serde(rename="h")]
    pub height: u32,
    #[serde(rename="b")]
    pub boxes: Vec<MemeTplTextBox>,
    #[serde(rename="n")]
    pub name: String,
    #[serde(rename="d")]
    pub description: String,
    #[serde(rename="u")]
    pub usage: String,
    #[serde(rename="k")]
    pub keywords: Vec<String>,
}