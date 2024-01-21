use std::{collections::BTreeMap, ops::Range};

use self::parsing::parse_almanac;
use crate::error::AocError;

pub struct AlmanacLine {
    dst: Range<u64>,
    src: Range<u64>,
}

impl AlmanacLine {
    pub fn new(dst: u64, src: u64, len: u64) -> Self {
        Self {
            dst: dst..dst + len,
            src: src..src + len,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct AlmanacCategory(BTreeMap<u64, u64>);

impl AlmanacCategory {
    pub fn new(lines: Vec<AlmanacLine>) -> Self {
        let inner = lines
            .into_iter()
            .flat_map(|line| line.src.zip(line.dst).collect::<Vec<_>>())
            .collect::<BTreeMap<u64, u64>>();

        Self(inner)
    }

    pub fn look_up(&self, n: u64) -> u64 {
        *self.0.get(&n).unwrap_or(&n)
    }
}

#[derive(PartialEq, Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    categories: Vec<AlmanacCategory>,
}

impl Almanac {
    pub fn new(seeds: Vec<u64>, categories: Vec<AlmanacCategory>) -> Self {
        Self { seeds, categories }
    }
}

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

    fn almanac(input: &str) -> IResult<&str, Almanac> {
        let seeds = preceded(tag("seeds: "), separated_list1(space1, u64));
        let map = tuple((terminated(u64, space1), terminated(u64, space1), u64));
        let maps = separated_list1(line_ending, map);
        let category_name = many1(alt((alphanumeric1, tag("-"))));
        let skip_line = preceded(tuple((category_name, tag(" map:"))), line_ending);
        let category = preceded(skip_line, maps);
        let categories = separated_list1(tuple((line_ending, line_ending)), category);

        let almanac = tuple((
            terminated(seeds, line_ending),
            preceded(line_ending, categories),
        ));

        map_res(almanac, |(seeds, categories)| {
            let categories = categories
                .into_iter()
                .map(|lines| {
                    let lines = lines
                        .into_iter()
                        .map(|(dst, src, len)| AlmanacLine::new(dst, src, len))
                        .collect::<Vec<_>>();

                    AlmanacCategory::new(lines)
                })
                .collect::<Vec<_>>();

            //
            let almanac = Almanac::new(seeds, categories);
            Ok::<Almanac, ()>(almanac)
        })(input)
    }

    pub fn parse_almanac(input: &str) -> Result<Almanac, AocError> {
        almanac(input)
            .map(|x| x.1)
            .map_err(|_| AocError::ParseAlmanacError)
    }
}

fn seed_location(seed: &u64, almanac: &Almanac) -> u64 {
    almanac
        .categories
        .iter()
        .fold(*seed, |n, category| category.look_up(n))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let almanac = parse_almanac(input)?;

    let closest = almanac
        .seeds
        .iter()
        .map(|seed| seed_location(seed, &almanac))
        .min()
        .unwrap_or(0);

    Ok(closest.to_string())
}

#[cfg(test)]
mod tests {
    use miette::IntoDiagnostic;

    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse_almanac() -> miette::Result<()> {
        let parsed = parsing::parse_almanac(INPUT).into_diagnostic()?;
        assert_eq!(parsed.seeds.len(), 4);
        assert_eq!(parsed.categories.len(), 7);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("35", process(INPUT)?);
        Ok(())
    }
}
