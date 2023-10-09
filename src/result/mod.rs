use super::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) trait ResultExt {

    fn err_prefix(self, prefix: impl AsRef<str>) -> Self;
}

impl<T> ResultExt for std::result::Result<T, Error> {

    fn err_prefix(self, prefix: impl AsRef<str>) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.prefix(prefix)),
        }
    }
}