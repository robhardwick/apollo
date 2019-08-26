use failure::Fail;

#[derive(Clone, Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "Unknown configuration preset: {}", 0)]
    UnknownPreset(String),

    #[fail(display = "No scale tags in preset configuration")]
    NoScaleTag,

    #[fail(display = "No scales with tag in configuration: {}", 0)]
    InvalidScaleTag(String),
}