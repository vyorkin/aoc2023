use crate::error::AocError;

#[derive(PartialEq, Debug)]
pub struct AlmanacMap {
    dst: u32,
    src: u32,
    len: u32,
}

impl AlmanacMap {
    pub fn new(dst: u32, src: u32, len: u32) -> Self {
        Self { dst, src, len }
    }
}

#[derive(PartialEq, Debug)]
pub struct Almanac {
    seeds: Vec<u32>,
    categories: Vec<Vec<AlmanacMap>>,
}

impl Almanac {
    pub fn new(seeds: Vec<u32>, categories: Vec<Vec<AlmanacMap>>) -> Self {
        Self { seeds, categories }
    }
}

mod parsing {
    use super::*;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, line_ending, newline, space1, u32},
        combinator::{map_res, rest},
        multi::{many1, separated_list1},
        sequence::{delimited, preceded, separated_pair, terminated, tuple},
        Err, IResult,
    };

    fn almanac(input: &str) -> IResult<&str, Almanac> {
        let seeds = preceded(tag("seeds: "), separated_list1(space1, u32));
        let map = tuple((terminated(u32, space1), terminated(u32, space1), u32));
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
                .map(|c| {
                    c.into_iter()
                        .map(|(dst, src, len)| AlmanacMap::new(dst, src, len))
                        .collect()
                })
                .collect::<Vec<_>>();
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

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    todo!("day 05 - part 1");
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
        let expected = Almanac::new(
            vec![79, 14, 55, 13],
            vec![
                vec![AlmanacMap::new(50, 98, 2), AlmanacMap::new(52, 50, 48)],
                vec![
                    AlmanacMap::new(0, 15, 37),
                    AlmanacMap::new(37, 52, 2),
                    AlmanacMap::new(39, 0, 15),
                ],
                vec![
                    AlmanacMap::new(49, 53, 8),
                    AlmanacMap::new(0, 11, 42),
                    AlmanacMap::new(42, 0, 7),
                    AlmanacMap::new(57, 7, 4),
                ],
                vec![AlmanacMap::new(88, 18, 7), AlmanacMap::new(18, 25, 70)],
                vec![
                    AlmanacMap::new(45, 77, 23),
                    AlmanacMap::new(81, 45, 19),
                    AlmanacMap::new(68, 64, 13),
                ],
                vec![AlmanacMap::new(0, 69, 1), AlmanacMap::new(1, 0, 69)],
                vec![AlmanacMap::new(60, 56, 37), AlmanacMap::new(56, 93, 4)],
            ],
        );

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_process() -> miette::Result<()> {
        assert_eq!("", process(INPUT)?);
        Ok(())
    }
}
