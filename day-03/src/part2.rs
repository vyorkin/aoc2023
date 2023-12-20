use crate::error::AocError;
use itertools::Itertools;
use std::collections::BTreeMap;

type CharMatrix = Vec<Vec<char>>;

struct EngineSchema(pub CharMatrix);

impl EngineSchema {
    const OFFSETS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    pub fn new(input: &str) -> Self {
        let result = input
            .lines()
            .map(|line| format!(".{}.", line).chars().collect())
            .collect();

        Self(result)
    }

    fn numbers(&self) -> BTreeMap<(usize, usize), u32> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(i, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .fold((Vec::new(), None), |(mut acc, maybe_digit), (j, &char)| {
                        if let Some(prev_digit) = maybe_digit {
                            if let Some(n) = char.to_digit(10) {
                                (acc, Some(prev_digit * 10 + n))
                            } else {
                                acc.push(((i, j - 1), prev_digit));
                                (acc, None)
                            }
                        } else if let Some(n) = char.to_digit(10) {
                            (acc, Some(n))
                        } else {
                            (acc, None)
                        }
                    })
                    .0
            })
            .collect::<BTreeMap<(usize, usize), u32>>()
    }

    fn gear_candidates(&self) -> Vec<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(i, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .filter(|(_, &c)| Self::is_gear_marker(c))
                    .map(move |(j, _)| (i, j))
            })
            .collect::<Vec<(usize, usize)>>()
    }

    pub fn gears(&self) -> Vec<u32> {
        let numbers = self.numbers();
        self.gear_candidates()
            .into_iter()
            .filter_map(|(i, j)| {
                let r = Self::OFFSETS
                    .iter()
                    .unique()
                    .fold(Vec::new(), |mut gears_acc, (di, dj)| {
                        let row = i as i32 + di;
                        let col = j as i32 + dj;

                        let adjacent =
                            numbers
                                .iter()
                                .fold(Vec::new(), |mut num_acc, (&(ni, nj), &n)| {
                                    let num_length = n.to_string().len() as i32;
                                    let num_row = ni as i32;
                                    let num_col_start = nj as i32 - num_length;
                                    let num_col_end = nj as i32;

                                    if num_row == row && num_col_start < col && col <= num_col_end {
                                        num_acc.push(n);
                                    }

                                    num_acc
                                });

                        gears_acc.push(adjacent);
                        gears_acc
                    })
                    .into_iter()
                    .flatten()
                    .unique()
                    .collect::<Vec<_>>();

                if r.len() == 2 {
                    Some(r.iter().product::<u32>())
                } else {
                    None
                }
            })
            .collect::<Vec<u32>>()
    }

    #[inline]
    fn is_gear_marker(c: char) -> bool {
        c == '*'
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let engine_schema = EngineSchema::new(input);
    let gears = engine_schema.gears();
    Ok(gears.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process0() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
