use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("cannot parse `{0}` as Card")]
    #[diagnostic(code(aoc::parser_card_error))]
    ParseCardError(String),
}
