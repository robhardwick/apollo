use failure::Fail;

#[derive(Clone, Debug, Fail)]
pub enum SynthError {
    #[fail(display = "Unable to get synthesizer signal type")]
    SignalType,
}