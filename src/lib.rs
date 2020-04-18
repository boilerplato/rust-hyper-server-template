pub use self::error::{Error, ErrorExt, ResultExt};

#[macro_use]
pub mod logger;
#[macro_use]
pub mod macros;

pub mod constants;
mod error;
pub mod prelude;
mod test_env;
pub mod utils;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_foo() {
        setup_test_env!();
        assert_eq!(1, 1);
    }
}
