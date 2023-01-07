use std;
use std::io;
use std::fmt;
use std::result;

use git2;


/// Enumeration with error categories
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub(crate) enum ErrorCategory {
    /// Unspecified error
    Generic,

    /// Git error
    Git,

    /// Error from operating system
    Os,

    /// Repository error
    Repo,
}


/// Structure, that describes all errors in mm
#[derive(Debug, PartialEq)]
pub(crate) struct Error {
    msg: String,
    category: ErrorCategory,
}


impl Error {
    /// Constructs an error from string description and specific category.
    /// 
    /// * `s` - string with error description
    /// * `category` - error category (see [`crate::error::ErrorCategory`])
    pub(crate) fn from_string(s: &str, category: ErrorCategory) -> Self {
        Error {
            msg: s.to_owned(),
            category: category
        }
    }


    /// Constructs an error from an error object.
    /// 
    /// * `err` - generic error instance (needs to implement [`std::error::Error`] trait)
    fn from_error<E: std::error::Error>(err: E) -> Self {
        Error {
            msg: err.to_string(),
            category: ErrorCategory::Generic
        }
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
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


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        let mut res = Error::from_error(err);
        res.category = ErrorCategory::Os;
        res
    }
}


impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        let mut res = Error::from_error(err);
        res.category = ErrorCategory::Git;
        res
    }
}


/// Crate-specific alias for [`std::result::Result`] instantiated 
/// with [`crate::error::Error`]
pub(crate) type Result<T> = result::Result<T, Error>;
