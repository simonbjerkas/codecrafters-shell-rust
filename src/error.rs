use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("{0}")]
    Execution(String),
    #[error("{0:?}: Failed to write to file")]
    WriteFile(std::fs::File),
    #[error("{0}: Failed to create file")]
    CreateFile(String),
    #[error("{0}: Failed to open file")]
    OpenFile(String),
}
