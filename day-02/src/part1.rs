use crate::error::AocError;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Vec<(u32, Color)>>,
}

impl Game {
    pub fn new(id: u32, rounds: Vec<Vec<(u32, Color)>>) -> Self {
        Self { id, rounds }
    }
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{space0, u32},
        combinator::{map_res, value},
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
        IResult,
    };

    pub fn parse_game(input: &str) -> IResult<&str, Game, ()> {
        // Game 13: 7 blue, 8 red; 5 green, 15 blue, 2 red; 7 green, 3 blue, 12 red

        let color = alt((
            value(Color::Red, tag::<&str, &str, ()>("red")),
            value(Color::Green, tag::<&str, &str, ()>("green")),
            value(Color::Blue, tag::<&str, &str, ()>("blue")),
        ));

        let colored_cubes = tuple((u32, preceded(space0, color)));
        let colored_cubes_list = separated_list1(tag(", "), colored_cubes);
        let game_round = separated_list1(tag("; "), colored_cubes_list);
        let game_id = delimited(tag("Game "), u32, tag(": "));
        let game = tuple((game_id, game_round));

        map_res(game, |(id, rounds)| Ok::<Game, ()>(Game::new(id, rounds)))(input)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (red, green, blue) = (12, 13, 14);

    let is_possible_game = |game: &Game| -> bool {
        game.rounds.iter().flatten().all(|&(n, color)| match color {
            Color::Red => n <= red,
            Color::Green => n <= green,
            Color::Blue => n <= blue,
        })
    };

    let games = input
        .lines()
        .map(|line| {
            parser::parse_game(line)
                .map(|x| x.1)
                .map_err(|_| AocError::ParseGameError(line.to_string()))
        })
        .collect::<Result<Vec<Game>, _>>()?;

    let sum = games
        .into_iter()
        .filter(is_possible_game)
        .map(|game| game.id)
        .sum::<u32>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;

    use super::*;

    #[test]
    fn test_parser() -> miette::Result<()> {
        let input = "Game 13: 7 blue, 8 red; 5 green, 15 blue, 2 red; 7 green, 3 blue, 12 red";
        let expected = Game::new(
            13,
            vec![
                vec![(7, Color::Blue), (8, Color::Red)],
                vec![(5, Color::Green), (15, Color::Blue), (2, Color::Red)],
                vec![(7, Color::Green), (3, Color::Blue), (12, Color::Red)],
            ],
        );
        let (_, game) = parser::parse_game(input).into_diagnostic()?;
        assert_eq!(game, expected);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("8", process(input)?);

        Ok(())
    }
}
