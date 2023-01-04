use std;
use std::io;
use std::fmt;

use git2;


/// Enumeration with error categories
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
enum ErrorCategory {
    /// Unspecified error
    Generic,

    /// Git error
    Git,

    /// Error from operating system
    Os,
}


/// Structure, that describes all errors in mm
#[derive(Debug, PartialEq)]
pub(crate) struct Error {
    msg: String,
    category: ErrorCategory,
}


impl Error {
    /// Constructs an error from an error object
    pub(crate) fn from_error<E: std::error::Error>(err: E) -> Self {
        Error {
            msg: err.to_string(),
            category: ErrorCategory::Generic
        }
    }

    /// Constructs an error from an I/O error object
    pub(crate) fn from_io_error(err: io::Error) -> Self {
        let mut res = Error::from_error(err);
        res.category = ErrorCategory::Os;
        res
    }

    /// Constructs an error from a git2 error object
    pub(crate) fn from_git_error(err: git2::Error) -> Self {
        let mut res = Error::from_error(err);
        res.category = ErrorCategory::Git;
        res
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        //
        // intentionally ignore category (maybe for now)
        //

        write!(f, "Description: {} (category: {:?})", self.msg, self.category)
    }
}


impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}