use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MemeTplFile {
    pub name: String,
    #[serde(rename="mime-type")]
    pub mime_type: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MemeTplDim {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MemeTplTextBox {
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub left: f32,
    pub rotation: Option<i32>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MemeTpl {
    pub id: String,
    pub num: u32,
    pub file: MemeTplFile,
    pub description: String,
    pub usage: String,
    pub keywords: Vec<String>,
    pub dim: MemeTplDim,
    pub boxes: Vec<MemeTplTextBox>,
}