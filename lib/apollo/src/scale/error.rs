use failure::Fail;

#[derive(Clone, Debug, Fail)]
pub enum ScaleError {
    #[fail(display = "Unable to create weighted distribution for scale notes")]
    DistributionCreate,
}