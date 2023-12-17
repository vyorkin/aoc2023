use crate::error::AocError;

#[allow(dead_code)]
mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, u32},
        combinator::value,
        multi::{many0, separated_list1},
        sequence::delimited,
        IResult,
    };

    pub fn parse_line(line: &str) -> IResult<&str, Vec<u32>> {
        delimited(
            many0(alpha1),
            separated_list1(
                many0(alpha1),
                alt((
                    u32,
                    value(1, tag("one")),
                    value(2, tag("two")),
                    value(3, tag("three")),
                    value(4, tag("four")),
                    value(5, tag("five")),
                    value(6, tag("six")),
                    value(7, tag("seven")),
                    value(8, tag("eight")),
                    value(9, tag("nine")),
                )),
            ),
            many0(alpha1),
        )(line)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha0, alpha1, digit1, u32},
        combinator::{map_res, value},
        multi::{many0, many1, separated_list1},
        sequence::{delimited, separated_pair, tuple},
        IResult,
    };
    use rstest::rstest;

    use super::*;

    #[allow(dead_code)]
    #[rstest]
    // #[case("29lzrxseven", "xxx")]
    // #[case("111xxx222", "xxx")]
    #[case("twonethree", "xxx")]
    fn test_parsing_separated_list1(
        #[case] line: &str,
        #[case] _expected: &str,
    ) -> miette::Result<()> {
        fn parse_word(input: &str) -> IResult<&str, u32> {
            alt((
                value(1, tag("one")),
                value(2, tag("two")),
                value(3, tag("three")),
            ))(input)
        }

        fn _parse0(line: &str) -> IResult<&str, Vec<&str>> {
            many0(delimited(many0(digit1), tag("xxx"), many0(digit1)))(line)
        }

        let mut parse1 = many1(tuple((alpha1, parse_word, alpha1)));

        let (_remaining, parsed) = parse1(line).map_err(|_| AocError::ParsingError)?;
        let result: Vec<u32> = parsed.into_iter().map(|(_, num, _)| num).collect();
        // dbg!(parsed);
        // dbg!(remaining);

        assert_eq!(result, vec![23]);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_parser() -> miette::Result<()> {
        let input = "29lzrxseven";
        let (remaining, parsed) = parser::parse_line(input).map_err(|_| AocError::ParsingError)?;
        dbg!(parsed);
        dbg!(remaining);
        Ok(())
    }

    #[test]
    #[ignore]
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
