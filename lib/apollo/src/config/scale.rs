use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigScale {
    pub name: String,
    pub tags: Vec<String>,
    pub notes: Vec<f32>,
}