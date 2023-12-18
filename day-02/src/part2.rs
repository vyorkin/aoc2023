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

    fn all_rounds(&self) -> impl Iterator<Item = (u32, Color)> {
        self.rounds.clone().into_iter().flatten()
    }

    fn how_possible(&self) -> (u32, u32, u32) {
        self.all_rounds().fold((0, 0, 0), |acc, (n, color)| {
            let (r, g, b) = acc;
            match color {
                Color::Red => (r.max(n), g, b),
                Color::Green => (r, g.max(n), b),
                Color::Blue => (r, g, b.max(n)),
            }
        })
    }

    pub fn power(&self) -> u32 {
        let (r, g, b) = self.how_possible();
        r * g * b
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

    fn game(input: &str) -> IResult<&str, Game, ()> {
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

    pub fn parse_game(input: &str) -> Result<Game, AocError> {
        game(input)
            .map(|x| x.1)
            .map_err(|_| AocError::ParseGameError(input.to_string()))
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let games = input
        .lines()
        .map(parser::parse_game)
        .collect::<Result<Vec<Game>, _>>()?;

    let sum = games.iter().map(Game::power).sum::<u32>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", process(input)?);

        Ok(())
    }
}
