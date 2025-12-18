use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("{0}")]
    Execution(String),
    #[error("{0:?}: Failed to write to file")]
    WriteFile(std::fs::File),
    #[error("{0}: Failed to create file")]
    CreateFile(String),
    #[error("Missing endquote")]
    MissingQuote,
    #[error("End of line error")]
    Eol,
    #[error("Missing argument")]
    MissingArg,
    #[error("Failed to parse command")]
    Parsing,
}
