use crate::error::AocError;

#[tracing::instrument]
fn recover_calibration_value(line: &str) -> Result<u32, AocError> {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next();
    let last = digits.last().or(first);
    let (Some(x), Some(y)) = (first, last) else {
        return Err(AocError::NoCalibrationValue(line.to_string()));
    };
    format!("{x}{y}")
        .parse()
        .map_err(|_| AocError::CannotParseAsNumber(line.to_string()))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    input
        .lines()
        .map(recover_calibration_value)
        .sum::<Result<u32, _>>()
        .map(|r| r.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("two1nine", "29")]
    #[case("eightwothree", "83")]
    #[case("abcone2threexyz", "13")]
    #[case("xtwone3four", "24")]
    #[case("4nineeightseven2", "42")]
    #[case("zoneight234", "14")]
    #[case("7pqrstsixteen", "76")]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
