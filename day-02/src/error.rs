use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("cannot parse `{0}` as number")]
    #[diagnostic(code(aoc::cannot_parse_as_number))]
    CannotParseAsNumber(String),

    #[error("cannot parse `{0}` as Game")]
    #[diagnostic(code(aoc::parser_game_error))]
    ParseGameError(String),
}
