use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("no calibration value in `{0}`")]
    #[diagnostic(code(aoc::no_calibration_value))]
    NoCalibrationValue(String),

    #[error("cannot parse `{0} as number")]
    #[diagnostic(code(aoc::cannot_parse_as_number))]
    CannotParseAsNumber(String),

    #[error("parsing error")]
    #[diagnostic(code(aoc::parsing_error))]
    ParsingError,
}
