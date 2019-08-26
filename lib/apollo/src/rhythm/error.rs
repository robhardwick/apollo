use failure::Fail;

#[derive(Clone, Debug, Fail)]
pub enum RhythmError {
    #[fail(display = "Unable to choose time signature unit")]
    UnitChoose,

    #[fail(display = "Unable to create weighted distribution for beat lengths")]
    DistributionCreate,
}