use core::fmt::{self, Display};

/// A compatibility wrapper around an error type from this crate.
///
/// `Compat` implements `std::error::Error`, allowing the types from this
/// crate to be passed to interfaces that expect a type of that trait.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Compat<E> {
    pub(crate) error: E,
}

impl<E: Display> Display for Compat<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl<E> Compat<E> {
    /// Unwrap this into the inner error.
    pub fn into_inner(self) -> E {
        self.error
    }
}

with_std! {
    use std::fmt::Debug;
    use std::error::Error as StdError;

    use Error;

    impl<E: Display + Debug> StdError for Compat<E> {
        fn description(&self) -> &'static str {
            "An error has occurred."
        }
    }

    impl From<Error> for Box<StdError> {
        fn from(error: Error) -> Box<StdError> {
            Box::new(Compat { error })
        }
    }
}
