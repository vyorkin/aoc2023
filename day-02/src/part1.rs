use crate::error::AocError;

enum Game {
    Single(u32),
    Double(u32),
    Triple(u32),
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, space0, space1},
        combinator::map_res,
        multi::many1,
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
    };

    pub fn parse_game(input: &str) -> IResult<&str, u32> {
        // Game 13: 7 blue, 8 red; 5 green, 15 blue, 2 red; 7 green, 3 blue, 12 red

        // let color = alt((tag("red"), tag("green"), tag("blue")));
        // let cubes = terminated(digit1, tuple((space0, color)));
        // let comma = tag(", ");
        // let round = tuple((cubes, comma, cubes, comma, cubes, tag(";"), space0));

        let (remaining, game_id) =
            map_res(delimited(tag("Game "), digit1, tag(": ")), |s: &str| {
                s.parse::<u32>()
            })(input)?;

        // let (a, b) = many1(round)(remaining);

        // let parse = (round, round, round);

        //
        // let result = parse(input);
        // let result = result.map_res(|_| Game::Single(0))?;
        Ok(("", 1))
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    todo!("day 02 - part 1");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
