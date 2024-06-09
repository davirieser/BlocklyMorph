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
    fn test_int() -> Result<()> {
        let int: usize = 420;

        let string = to_string(&int)?;
        let value = from_str::<usize>(&string)?;

        assert!(int == value);

        Ok(())
    }
}
