use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("cannot parse Almanac")]
    #[diagnostic(code(aoc::parse_almanac_error))]
    ParseAlmanacError,
}
