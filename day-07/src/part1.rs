use crate::error::AocError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A = 1,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: u32) -> Self {
        Self { cards, bid }
    }

    pub fn hand_type(&self) -> HandType {
        todo!();
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    OfKind(usize, Card),
    FullHouse(Card, Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card),
}

// impl Ord for HandType {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         match (self, other) {
//             (HandType::HighCard(a), HandType::HighCard(b)) => a.partial_cmp(b),
//             (_, _) => None,
//         }
//     }
// }
//
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

    pub fn cards(input: &str) -> IResult<&str, Vec<Card>> {
        // T55J5 684
        todo!();
    }

    pub fn hands(input: &str) -> IResult<&str, Vec<Hand>> {
        todo!();
    }

    pub fn parse_hands(input: &str) -> Result<Vec<Hand>, AocError> {
        hands(input)
            .map(|x| x.1)
            .map_err(|_| AocError::ParseHandsError)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // let mut hands = parsing::parse_hands(input)?;
    // hands.sort_by(|a, b| a.hand_type().cmp(&b.hand_type()));

    let ranked = todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;

    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test_log::test]
    fn test_parse_hands() -> miette::Result<()> {
        let input = "T55J5 684\nKK677 28";
        let parsed = parsing::parse_hands(input).into_diagnostic()?;
        let expected = vec![
            Hand::new(
                vec![Card::T, Card::Five, Card::Five, Card::J, Card::Five],
                684,
            ),
            Hand::new(
                vec![Card::K, Card::K, Card::Six, Card::Seven, Card::Seven],
                28,
            ),
        ];

        Ok(())
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("", process(INPUT)?);
        Ok(())
    }
}
