use std::io::Error;


#[derive(thiserror::Error, Debug)]

/// Simple errors, like FileError which is converted from `[std::io::Error]`
/// ArgNotFound error is given when you run the `commot` command without giving the file or when the required arg is not 
/// provided by the user.
pub enum CommotError {
    #[error("Something went wrong with the file.")]
    FileError(#[from] Error),
    #[error("{0}")]
    ArgNotFound(String)
}