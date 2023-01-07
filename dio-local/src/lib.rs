//! This module contains the actual `count_words` implementation.
//!
//! Such code uses the regular [`Result<T, E>`] type to provide interoperability, rather than
//! forcing people to adopt a specific error handling library.

pub mod errors;

use errors::DataStoreError;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

//-------------------------------------------------------------------------------------------------

/// `read_file` reads data into a buffer from `input` file and writes content to a string.
///
/// If successful this function returns the contents of the file as a `String`.
///
/// # Errors
///
/// If the data in the `input` file stream is *not* valid UTF-8
/// then an error is returned and `BufReader` made from `input` is unchanged.
///
/// # Examples
///
/// [`File`]s implement `Read`:
///
/// ```no_run
/// # use dio_local::{read_file};
/// # use std::io::{self, Cursor, ErrorKind};
/// #
/// # fn main() -> Result<(), miette::Error> {
/// let mut file = Cursor::new(String::from("foobar"));
/// let content: String = read_file(&mut file)?;
/// assert_eq!(content, "foobar");
/// # Ok(())
/// # }
/// ```
pub fn read_file<R: Read>(input: &mut R) -> Result<String, DataStoreError> {
    let mut data = String::new();
    BufReader::new(input).read_to_string(&mut data)?;
    Ok(data)
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Data {
    pub id: u32,
    pub title: String,
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn get_json<R: Read>(
    path: Option<&str>,
    file: Option<&mut R>,
) -> Result<Vec<Data>, DataStoreError> {
    match (path, file) {
        (None, None) => return Err(DataStoreError::EmptySource),
        (None, Some(f)) => {
            let string: String = read_file(f)?;
            let data: Vec<Data> = deserialize(&string)?;
            return Ok(data);
        }
        (Some(p), None) => {
            let mut f = File::open(p)?;
            let string: String = read_file(&mut f)?;
            let data: Vec<Data> = deserialize(&string)?;
            return Ok(data);
        }
        (Some(p), Some(_f)) => {
            let mut f = File::open(p)?;
            let string: String = read_file(&mut f)?;
            let data: Vec<Data> = deserialize(&string)?;
            return Ok(data);
        }
    };
}

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
pub fn deserialize(string: &str) -> Result<Vec<Data>, DataStoreError> {
    Ok(serde_json::from_str(string).unwrap())
}

//-------------------------------------------------------------------------------------------------

/// `count_words`
///
/// * Iterating on reader returns `Result<String>` because read operations can fail.
/// * If result is `Err` variant, `map_err()` transforms the error value inside result from
///   `io::Error` into a `DataStoreError:ReadError`.
/// * If the result is of `Ok` variant, it remains unchanged.
/// * Then unpack the result with the `?` operator for to match cases:
///   * `Ok` - Assign the result to variable `line`.
///   * `Err` - Function exits here, returning this as the return value.
/// * Since `io:Error` is encapsulated under the `source` attribute of `DataStoreError::ReadError` ,
///   the context/error chain remains intact. This enables `anyhow`, used in application side of
///   this library ends up displaying both errors.
///
/// If we didn't care for the specialized WordCountError::ReadError variant, this means we could
/// have also written our code as follows, in which case we no longer need to use map_err() and can
/// use ? directly:
///
/// ```ignore
/// for line in reader.lines() {
///    for _word in line?.split_whitespace() {
///        word_count += 1;
///    }
///}
/// ```
///
/// The `Read` trait allows for reading bytes from a source.
///
/// # Examples
///
/// [`File`]s implement `Read`:
///
/// [`Ok(n)`]: Ok
/// [`File`]: crate::fs::File
/// [`TcpStream`]: crate::net::TcpStream
///
/// ```no_run
/// # use dio_local::count_words;
/// # use std::io::{self, Cursor, ErrorKind};
///
/// # fn main() {
///     let mut file = Cursor::new(String::from("foobar"));
///     let word_count = count_words(&mut file).expect("should count words from File or Cursor");
///     assert_eq!(word_count, 1u32);
/// # }
/// ```
pub fn count_words<R: Read>(input: &mut R) -> Result<u32, DataStoreError> {
    let reader: BufReader<&mut R> = BufReader::new(input);
    let mut word_count = 0u32;
    for line in reader.lines() {
        let line: String = line.map_err(|source| DataStoreError::ReadError { source })?;
        for _word in line.split_whitespace() {
            word_count += 1u32;
        }
    }

    if word_count == 0u32 {
        return Err(DataStoreError::EmptySource);
    }

    Ok(word_count)
}

/// * `path` - Path to file. Defaults to 'data.json'.
pub fn get_file(path: Option<&str>) -> Result<(), DataStoreError> {
    let mut input = File::open(path.unwrap_or("data.json"))?;

    if count_words(&mut input)? == 0 {
        return Err(DataStoreError::EmptySource);
    }

    Ok(())
}

//-------------------------------------------------------------------------------------------------

/// Copied (with little modification from) -> [Source](https://github.com/zoni/rust-wordcount/blob/36f4545f2b994f054a17ac31df07e2f762c6ac24/src/wordcounter.rs#L124-L125)
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor, ErrorKind};

    //------------------------------------------------------------------------------

    /// An implementation of `std::io::Read` that fails on the first call to `read` and
    /// throws an `std::io::Error` with the given ErrorKind and message.
    #[derive(Debug, PartialEq)]
    pub struct ErrReader<'a> {
        /// The ErrorKind to put into the `std::io::Error`.
        pub kind: ErrorKind,
        pub msg: &'a str,
    }

    impl<'a> io::Read for ErrReader<'a> {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(self.kind, self.msg))
        }
    }

    impl<'a> ErrReader<'a> {
        /// Construct a new ErrReader.
        pub fn new(kind: ErrorKind, msg: &'a str) -> Self {
            Self { kind, msg }
        }
    }

    //------------------------------------------------------------------------------

    #[test]
    fn it_read_file() {
        let mut file = Cursor::new(String::from("foobar"));
        let content: String = read_file(&mut file).expect("should read and return content of file");
        assert_eq!(content, "foobar");
    }

    #[test]
    fn it_deserialize_to_json() {
        let input: &str = r#"[{"id": 0, "title": "Hello" }, { "id": 1, "title": "World" }]"#;
        let mut input = Cursor::new(input);
        let content: String = read_file(&mut input).expect("should read Cursor or File");
        let data: Vec<Data> = deserialize(&content).expect("should return Vec<Data> from file");

        #[rustfmt::skip]
        assert_eq!( data[0], Data { id: 0u32, title: String::from("Hello") });
        #[rustfmt::skip] 
        assert_eq!( data[1], Data { id: 1u32, title: String::from("World") });
    }

    #[test]
    fn it_get_json() {
        let input: &str = r#"[{"id": 0, "title": "Hello" }, { "id": 1, "title": "World" }]"#;
        let mut input = Cursor::new(input);
        let data = get_json(None, Some(&mut input)).expect("should return Vec<Data>");

        #[rustfmt::skip]
        assert_eq!( data[0], Data { id: 0u32, title: String::from("Hello") });
        #[rustfmt::skip] 
        assert_eq!( data[1], Data { id: 1u32, title: String::from("World") });
    }

    //------------------------------------------------------------------------------

    #[test]
    fn count_single_word() {
        let mut file = Cursor::new(String::from("foobar"));
        let word_count = count_words(&mut file).expect("should count words from File or Cursor");
        assert_eq!(word_count, 1u32);
    }

    #[test]
    fn count_multiple_words() {
        let mut file = Cursor::new(String::from("foobar \nbaz"));
        let word_count = count_words(&mut file).expect("should count words from File or Cursor");
        assert_eq!(word_count, 2u32);
    }

    #[test]
    #[should_panic]
    fn empty_input() {
        let mut file = Cursor::new(String::from(""));
        let word_count =
            count_words(&mut file).expect("should count words from File or Cursor if any");
        assert_eq!(word_count, 0u32); // Err => `EmptySource`.
                                      // assert_matches!(err, WordCountError::EmptySource { .. });
    }

    #[test]
    #[should_panic]
    fn read_broken_pipe() {
        let mut f = ErrReader::new(ErrorKind::BrokenPipe, "read: broken pipe");
        let _err = count_words(&mut f).expect("should count words");
        // assert_matches!(err, WordCountError::ReadError { .. });
    }
}

//-------------------------------------------------------------------------------------------------
