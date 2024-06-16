mod serializer;
pub use serializer::{to_string, Serializer};

mod deserializer;
pub use deserializer::{from_str, Deserializer};

pub mod definitions;

mod error;
pub use error::{Error, Result};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_null() -> Result<()> {
        let value : Option<usize> = None;

        let string = to_string(&value)?;
        let parsed = from_str::<Option<usize>>(&string)?;

        assert!(parsed == value);

        Ok(())
    }

    #[test]
    fn test_uint() -> Result<()> {
        let value: usize = 420;

        let string = to_string(&value)?;
        let parsed = from_str::<usize>(&string)?;

        assert!(parsed == value);

        Ok(())
    }

    #[test]
    fn test_int() -> Result<()> {
        let value : isize = -420;

        let string = to_string(&value)?;
        let parsed = from_str::<isize>(&string)?;

        assert!(parsed == value);

        Ok(())
    }
}
