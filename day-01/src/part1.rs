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
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // In this example, the calibration values of these four lines are:
        // 12, 38, 15, and 77
        // Adding these together produces 142

        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!("142", process(input)?);
        Ok(())
    }
}
