use std::collections::HashSet;

use crate::error::AocError;

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    chosen_numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    fn new(id: u32, chosen: &[u32], winning: &[u32]) -> Self {
        Self {
            id,
            chosen_numbers: chosen.iter().cloned().collect(),
            winning_numbers: winning.iter().cloned().collect(),
        }
    }

    fn points(&self) -> u32 {
        let matches = self
            .chosen_numbers
            .intersection(&self.winning_numbers)
            .count() as u32;

        if matches > 0 {
            2u32.pow(matches - 1)
        } else {
            0
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = AocError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let xs = [1, 2, 3];
        parsing::parse_card(input)
    }
}

mod parsing {
    use super::*;

    use miette::IntoDiagnostic;
    use nom::{
        bytes::complete::tag,
        character::complete::{space1, u32},
        combinator::map_res,
        multi::separated_list1,
        sequence::{delimited, preceded, separated_pair, tuple},
        IResult,
    };

    fn card(input: &str) -> IResult<&str, Card> {
        let card = tuple((
            delimited(tuple((tag("Card"), space1)), u32, tag(":")),
            preceded(
                space1,
                separated_pair(
                    separated_list1(space1, u32),
                    delimited(space1, tag("|"), space1),
                    separated_list1(space1, u32),
                ),
            ),
        ));

        map_res(card, |(id, (chosen, winning))| {
            let card = Card::new(id, &chosen, &winning);
            Ok::<Card, ()>(card)
        })(input)
    }

    pub fn parse_card(input: &str) -> Result<Card, AocError> {
        card(input)
            .map(|x| x.1)
            .map_err(|_| AocError::ParseCardError(input.to_string()))
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    input
        .lines()
        .try_fold(0, |acc, line| {
            let card = Card::try_from(line)?;
            Ok(acc + card.points())
        })
        .map(|x| x.to_string())
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;

    use super::*;

    #[test]
    fn test_parse_card() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let parsed = Card::try_from(input)?;
        let expected = Card::new(1, &[41, 48, 83, 86, 17], &[83, 86, 6, 31, 17, 9, 48, 53]);

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!("13", process(input)?);

        Ok(())
    }
}
