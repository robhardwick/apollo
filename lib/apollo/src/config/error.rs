use failure::Fail;

#[derive(Clone, Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "Unknown configuration preset: {}", 0)]
    UnknownPreset(String),
}