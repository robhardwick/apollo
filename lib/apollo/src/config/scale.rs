use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigScale {
    pub id: String,
    pub tags: Vec<String>,
    pub notes: Vec<f32>,
}