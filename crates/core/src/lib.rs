mod codegen;
mod error;
mod inflector;
mod target;

pub use codegen::codegen;
pub use error::{Error, Result};
pub use inflector::*;
pub use target::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
