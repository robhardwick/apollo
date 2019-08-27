use failure::Fail;

/// A configuration error
#[derive(Clone, Debug, Fail)]
pub enum ConfigError {
    /// An unknown preset ID was specified
    #[fail(display = "Unknown configuration preset: {}", 0)]
    UnknownPreset(String),

    /// No scale tags could be found in a preset configuration
    #[fail(display = "No scale tags in preset configuration")]
    NoScaleTag,

    /// No scales with a specified tag could be found
    #[fail(display = "No scales with tag in configuration: {}", 0)]
    InvalidScaleTag(String),
}