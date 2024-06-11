use thiserror::Error;

pub type Result<Ok, Err = Error> = std::result::Result<Ok, Err>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("nothing implemented")]
    NothingImplemented {},
}
