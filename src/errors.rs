#[derive(Debug)]
pub enum Error {
    ResourceLoadFailed,
    SetCoreOptionFailed,
    AsstCreateFailed,
    DyLibError(libloading::Error),
}

impl From<libloading::Error> for Error {
    fn from(err: libloading::Error) -> Self {
        Self::DyLibError(err)
    }
}
