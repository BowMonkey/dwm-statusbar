use thiserror::Error;
pub type Result<T, E = Errors> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Can not find Module {0}")]
    ModuleMissing(String),

    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ReadConfigFileError(#[from] std::io::Error),

    //#[error("invalid header (expected {expected:?}, found {found:?})")]
    //InvalidHeader { expected: String, found: String },
    #[error("unknown error")]
    Unknown,
}
