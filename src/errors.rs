use thiserror::Error;

#[derive(Error,Debug)]
pub enum Errors {
    #[error("test error")]
    TestError,
}