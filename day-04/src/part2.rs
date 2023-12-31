use std::collections::{BTreeMap, HashSet};

use crate::error::AocError;

//     id
// ------------------------------------------------
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

//   0       1       2       3       4       5
// Card 1, Card 2, Card 3, Card 4, Card 5, Card 6
// ------------------------------------------------
//           1       1       1       1
//                   2       2
//                           3       3
//                                   4
//

// 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 -> (4, [1, 4]))
// 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19 -> (2, [2, 3])
// 3: 1 21 53 59 44  | 69 82 63 72 16 21 14  1 -> (2, [3, 4])
// 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83 -> (1, [4, 4])
// 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36 -> (0, -)
// 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11 -> (0, -)

#[derive(Debug, PartialEq, Clone)]
struct Card {
    id: u32,
    chosen: HashSet<u32>,
    winning: HashSet<u32>,
}

impl Card {
    fn new(id: u32, chosen: HashSet<u32>, winning: HashSet<u32>) -> Self {
        Self {
            id,
            chosen,
            winning,
        }
    }

    pub fn number_of_matches(&self) -> u32 {
        self.chosen.intersection(&self.winning).count() as u32
    }
}

impl TryFrom<&str> for Card {
    type Error = AocError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        parsing::parse_card(input)
    }
}

mod parsing {
    use super::*;

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
            let chosen = chosen.into_iter().collect();
            let winning = winning.into_iter().collect();
            let result = Card::new(id, chosen, winning);
            Ok::<Card, ()>(result)
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
    let cards = input
        .lines()
        .map(Card::try_from)
        .collect::<Result<Vec<Card>, _>>()?;

    let copies = cards
        .iter()
        .map(|card| (card.id, 1))
        .collect::<BTreeMap<u32, u32>>();

    let total = cards
        .iter()
        .fold(copies, |mut acc, card| {
            let number_of_cards = *acc.get(&card.id).unwrap();

            let from = card.id + 1;
            let to = card.id + 1 + card.number_of_matches();

            for copy_id in from..to {
                acc.entry(copy_id).and_modify(|n| {
                    *n += number_of_cards;
                });
            }

            acc
        })
        .values()
        .sum::<u32>();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_parse_scratchcard() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let parsed = Card::try_from(input)?;
        let expected = Card::new(
            1,
            [41, 48, 83, 86, 17].into(),
            [83, 86, 6, 31, 17, 9, 48, 53].into(),
        );

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

        assert_eq!("30", process(input)?);

        Ok(())
    }
}
