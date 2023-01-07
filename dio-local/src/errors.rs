use miette::Diagnostic;
use thiserror::Error;

//-------------------------------------------------------------------------------------------------

/// `DataStoreError` enumerates all possible errors returned by this library.
///
/// `thiserror` provides a derive implementation which adds the `Error` trait for us.
/// As previously mentioned, to implement `Error` we have to implement `Display` and `thiserror'.
///
/// `#[error]` -  attributes provide ability to template for the displayed errors.
///
/// # References
///
/// * [error-handling](https://www.shuttle.rs/blog/2022/06/30/error-handling)
/// * [rust-error-handling](https://nick.groenen.me/posts/rust-error-handling/)
///
/// (Quoting the official documentation: "Thiserror deliberately does not appear in your public API.
/// You get the same thing as if you had written an implementation of std::error::Error by hand, and
/// switching from handwritten `impls` to thiserror or vice versa is not a breaking change.").
///
/// Users now get a lot more insight into the possible error cases that might be returned. As an
/// added benefit, we also no longer have to Box Error because the size of `DataStoreError`  can be
/// determined at compile time.
///
/// errors may use error(transparent) to forward the source and Display methods straight through
/// to an underlying error without adding an additional message.
#[derive(Error, Debug, Diagnostic)]
pub enum DataStoreError {
    /// Represents an empty source. For example, an empty text file being give to collect data from.
    #[error("Source contains no data")]
    EmptySource,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("unknown data store error")]
    Unknown,
    // None => Err(DataStoreError::NoFilePath),
    // NoFilePath,
}

//-------------------------------------------------------------------------------------------------

/*

#[error("the data for key `{0}` is not available")]
Redaction(String),

#[error("invalid header (expected {expected:?}, found {found:?})")]
InvalidHeader { expected: String, found: String },

#[error("data store disconnected")]
Disconnect(#[from] io::Error),

 */

//-------------------------------------------------------------------------------------------------

/// ```ignore
/// # // lib/error.rs
/// # use miette::Diagnostic;
/// # use thiserror::Error;
/// // ..
/// ```
///
/// [Reference](https://crates.io/crates/miette)
/// * miette is fully compatible with library usage. Consumers who don't know about, or don't want,
///   miette features can safely use its error types as regular std::error::Error.
/// * We highly recommend using something like thiserror to define unique error types and error
///   wrappers for your library.
/// * While miette integrates smoothly with thiserror, it is not required. If you don't want to use
///   the Diagnostic derive macro, you can implement the trait directly, just like with
///   std::error::Error.
/// * Then, return this error type from all your fallible public APIs. It's a best practice to wrap
///   any "external" error types in your error enum instead of using something like Report in a
///   library.
#[derive(Error, Diagnostic, Debug)]
pub enum LibError {
    #[error(transparent)]
    #[diagnostic(code(my_lib::io_error))]
    IOError(#[from] std::io::Error),

    #[error("Oops it blew up")]
    #[diagnostic(code(my_lib::bad_code))]
    BadThingHappened,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn it_lorem() {
        let err = LibError::IOError(io::Error::new(io::ErrorKind::Other, "lorem ipsum")); // -> `IOError(Custom { kind: Other, error: "lorem ipsum" })`
        assert_eq!(format!("{}", err), "lorem ipsum");

        let err = LibError::BadThingHappened; // -> `BadThingHappened`
        assert_eq!(format!("{}", err), "Oops it blew up");
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        let err = LibError::IOError(io::Error::new(io::ErrorKind::Other, "lorem ipsum"));
        panic!("{}", err);
    }

    #[test]
    fn it_works() -> Result<(), LibError> {
        Ok(())
    }

    #[test]
    fn it_match() {
        let err = LibError::IOError(io::Error::new(io::ErrorKind::Other, "lorem ipsum"));
        match err {
            LibError::IOError(x) => {
                assert_eq!(format!("{}", x), "lorem ipsum");
            }
            LibError::BadThingHappened => {
                unreachable!("{}", err);
            }
        }
    }
}
