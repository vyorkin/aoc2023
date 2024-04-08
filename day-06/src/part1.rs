use tracing::info;

use crate::error::AocError;

mod parsing {
    use std::iter::zip;

    use super::*;

    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, multispace1, u32},
        combinator::map_res,
        multi::separated_list1,
        sequence::{preceded, terminated, tuple},
        IResult,
    };

    fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
        preceded(multispace1, separated_list1(multispace1, u32))(input)
    }

    pub fn sheet(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
        let times = preceded(tag("Time:"), numbers);
        let distances = preceded(tag("Distance:"), numbers);

        let parse = tuple((terminated(times, line_ending), distances));
        map_res(parse, |(times, distances)| {
            Ok::<Vec<(u32, u32)>, &str>(zip(times, distances).collect())
        })(input)
    }

    pub fn parse_sheet(input: &str) -> Result<Vec<(u32, u32)>, AocError> {
        sheet(input)
            .map(|x| x.1)
            .map_err(|_| AocError::ParseSheetError)
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let sheet = parsing::parse_sheet(input)?;

    let result = sheet
        .into_iter()
        .map(|(time, dist)| {
            let count = (1..time)
                .filter(|speed| (time - speed) * speed > dist)
                .inspect(|speed| {
                    info!(
                        "({0} - {1}) * {1} = {3} > {2}",
                        time,
                        speed,
                        dist,
                        (time - speed) * speed
                    );
                })
                .count();
            (dist, count)
        })
        .inspect(|(dist, count)| info!("{0}: {1}", dist, count))
        .fold(1, |acc, x| acc * x.1);

    Ok(result)
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
        let expected = vec![(7, 9), (15, 40), (30, 200)];

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = INPUT;
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
