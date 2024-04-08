use crate::error::AocError;

mod parsing {
    use super::*;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, line_ending, space1, u64},
        combinator::map_res,
        multi::{many1, separated_list1},
        sequence::{preceded, terminated, tuple},
        IResult,
    };

    pub fn parse_sheet(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
        todo!()
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
