use std::{fs::File, io};

#[derive(Debug)]
pub(crate) struct DioErrors;

impl DioErrors {
    /// Prints an error message and exits the program.
    pub(crate) fn exit_out_of_bounds() {
        eprintln!("Index out of bounds");
        std::process::exit(1);
    }

    /// Unwrap a file open error
    ///
    /// # Arguments
    ///
    /// * `arg` - The file path
    /// * `e` - The error
    ///
    /// # Example
    ///
    /// ```
    /// let f = unwraperr_file_open("/tmp/foo.txt", &e);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be opened
    ///
    pub(crate) fn unwraperr_file_open(arg: &str, e: &io::Error) -> File {
        println!("{}", arg);
        println!("{}", e);
        panic!("Failed to open file");
    }
}
