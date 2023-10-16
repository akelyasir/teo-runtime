use teo_teon::Value;
use crate::error::Error;
use crate::object::Object;

impl<'a> TryFrom<&'a Object> for &'a Vec<Value> {

    type Error = Error;

    fn try_from(value: &'a Object) -> std::result::Result<Self, Self::Error> {
        let teon: &'a Value = value.try_into()?;
        match teon.as_array() {
            Some(v) => Ok(v),
            None => Err(Error::new(format!("object cannot convert to Vec<Value>: {:?}", value)))
        }
    }
}