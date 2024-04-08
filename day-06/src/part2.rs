use crate::error::AocError;

mod parsing {
    use super::*;

    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, line_ending, multispace1},
        combinator::map_res,
        multi::separated_list1,
        sequence::{preceded, terminated, tuple},
        IResult,
    };

    fn numbers(input: &str) -> IResult<&str, Vec<&str>> {
        preceded(multispace1, separated_list1(multispace1, digit1))(input)
    }

    pub fn sheet(input: &str) -> IResult<&str, (u64, u64)> {
        let times = preceded(tag("Time:"), numbers);
        let distances = preceded(tag("Distance:"), numbers);

        let parse = tuple((terminated(times, line_ending), distances));
        map_res(parse, |(times, distances)| {
            let time = times.join("").parse::<u64>().unwrap();
            let dist = distances.join("").parse::<u64>().unwrap();
            Ok::<(u64, u64), &str>((time, dist))
        })(input)
    }

    pub fn parse_sheet(input: &str) -> Result<(u64, u64), AocError> {
        sheet(input)
            .map(|x| x.1)
            .map_err(|_| AocError::ParseSheetError)
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let (time, dist) = parsing::parse_sheet(input)?;

    let count = (1..time)
        .filter(|speed| (time - speed) * speed > dist)
        .count();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;

    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test_log::test]
    fn test_parse_sheet() -> miette::Result<()> {
        let parsed = parsing::parse_sheet(INPUT).into_diagnostic()?;
        let expected = (71530, 940200);

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = INPUT;
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
