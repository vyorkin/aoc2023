#![allow(dead_code)]

use miette::{miette, Context, Diagnostic, IntoDiagnostic, LabeledSpan, SourceSpan};
use semver::Version;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug, Diagnostic)]
enum AocError {
    #[error(transparent)]
    #[diagnostic(code(day_01::io_error))]
    Io(#[from] std::io::Error),

    #[error("Critical failure")]
    #[diagnostic(code(day_01::critical_error))]
    Critical,

    #[error(transparent)]
    #[diagnostic(transparent)]
    Other(#[from] OtherError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("other error")]
struct OtherError {
    #[label("here")]
    pub at: SourceSpan,
}

#[derive(Error, Diagnostic, Debug)]
#[error("whatever error")]
#[diagnostic(help("try not trying instead"))]
struct WhateverError;

#[allow(dead_code)]
fn just_fail() -> miette::Result<()> {
    Err(WhateverError).into_diagnostic()
}

#[allow(dead_code)]
fn fail_version_parsing() -> miette::Result<Version> {
    let version = "1.0.x"
        .parse()
        .into_diagnostic()
        .wrap_err("Failed to parse semver version.")?;

    Ok(version)
}

#[allow(dead_code)]
fn fail_version_parsing_adhoc() -> miette::Result<Version> {
    let version = "1.2.x";
    let result = version
        .parse()
        .map_err(|_| miette!("Invalid version {}", version))?;
    Ok(result)
}

#[allow(dead_code)]
fn print_report() {
    let source = "2 + 2 * 2 = 8".to_string();
    let report = miette!(
        labels = vec![LabeledSpan::at(12..13, "this should be 6"),],
        help = "'*' has greater precedence than '+'",
        "Wrong answer"
    )
    .with_source_code(source);
    println!("{:?}", report)
}

fn demo_miette() {
    // print_report();

    // just_fail().wrap_err("you are zhostko naebalsya")?;

    // let _ = fail_version_parsing()?;
    // let _ = fail_version_parsing_adhoc()?;

    // let _v: Version = "2.5.x"
    //     .parse()
    //     .into_diagnostic()
    //     .wrap_err("Failed to parse semver version")?;
}

fn demo_include_str() {
    // let data = include_str!("./data.txt");
    // dbg!(data);
}

fn main() -> miette::Result<()> {
    // demo_miette();
    // demo_include_str();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::unfold;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, space0, u32},
        combinator::{map_res, value},
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
    };

    #[derive(Debug, PartialEq, Clone)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    #[test]
    fn test_vec_matrix() {
        let matrix: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let a23 = matrix[1][2];
    }

    #[test]
    fn test_parse_rgb_color() -> miette::Result<()> {
        let input = "7 blue, 8 red; 5 green, 15 blue, 2 red; 7 green, 3 blue, 12 red";

        let color = alt((
            value(Color::Red, tag::<&str, &str, ()>("red")),
            value(Color::Green, tag::<&str, &str, ()>("green")),
            value(Color::Blue, tag::<&str, &str, ()>("blue")),
        ));

        let colored_cubes = tuple((u32, preceded(space0, color)));
        let comma = tag(", ");
        let colored_cubes_list = separated_list1(comma, colored_cubes);
        let mut round_parser = separated_list1(tag("; "), colored_cubes_list);

        let (_remaining, cubes) = round_parser(input).into_diagnostic()?;

        assert_eq!(
            cubes,
            vec![
                vec![(7, Color::Blue), (8, Color::Red)],
                vec![(5, Color::Green), (15, Color::Blue), (2, Color::Red)],
                vec![(7, Color::Green), (3, Color::Blue), (12, Color::Red)]
            ]
        );

        Ok(())
    }

    #[test]
    fn test_elve_game() -> miette::Result<()> {
        let input = "Game 13: 7 blue, 8 red; 5 green, 15 blue, 2 red; 7 green, 3 blue, 12 red";

        let (_remaining, game_id) = map_res(
            delimited(tag("Game "), digit1::<&str, ()>, tag(": ")),
            |s: &str| s.parse::<u32>(),
        )(input)
        .into_diagnostic()
        .wrap_err("Unable to parse game")?;

        assert_eq!(game_id, 13);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_unfold_digit_names() {
        let input = "xtwone3four";
        let digits = unfold(String::from(input), |s| {
            let (result, skip) = if s.starts_with("one") {
                (Some(1), 3)
            } else if s.starts_with("two") {
                (Some(2), 3)
            } else if s.starts_with("three") {
                (Some(3), 5)
            } else if s.starts_with("four") {
                (Some(4), 4)
            } else if s.starts_with("five") {
                (Some(5), 4)
            } else if s.starts_with("six") {
                (Some(6), 3)
            } else if s.starts_with("seven") {
                (Some(7), 5)
            } else if s.starts_with("eight") {
                (Some(8), 5)
            } else if s.starts_with("nine") {
                (Some(9), 4)
            } else if let Some(c) = s.chars().next() {
                if let Some(n) = c.to_digit(10) {
                    (Some(n), 1)
                } else {
                    (Some(0), 1)
                }
            } else {
                (None, 0)
            };

            *s = String::from(&s[skip..]);

            result
        });
        let result = digits.filter(|&n| n != 0).collect::<Vec<_>>();
        assert_eq!(result, vec![]);
    }
}
