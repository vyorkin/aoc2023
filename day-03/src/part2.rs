use crate::error::AocError;
use itertools::Itertools;

type CharMatrix = Vec<Vec<char>>;

struct EngineSchematic {
    numbers: Vec<((usize, usize), u32)>,
    gear_candidates: Vec<(usize, usize)>,
}

impl EngineSchematic {
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
        let matrix = input
            .lines()
            .map(|line| format!("{}.", line).chars().collect())
            .collect();

        Self {
            numbers: Self::numbers(&matrix),
            gear_candidates: Self::gear_candidates(&matrix),
        }
    }

    pub fn calculate_gear_ratio_sum(&self) -> u32 {
        self.gear_ratios()
            .into_iter()
            .map(|(x, y)| x * y)
            .sum::<u32>()
    }

    pub fn gear_ratios(&self) -> Vec<(u32, u32)> {
        self.gear_candidates
            .iter()
            .filter_map(|&(i, j)| {
                let adjacent = self.adjacent_numbers(i, j);
                if adjacent.len() == 2 {
                    Some((adjacent[0], adjacent[1]))
                } else {
                    None
                }
            })
            .collect()
    }

    fn adjacent_numbers(&self, i: usize, j: usize) -> Vec<u32> {
        Self::OFFSETS
            .iter()
            .fold(Vec::new(), |mut acc, (di, dj)| {
                let row = i as i32 + di;
                let col = j as i32 + dj;
                acc.push(self.adjacent_numbers_at(row, col));
                acc
            })
            .into_iter()
            .flatten()
            .unique()
            .collect()
    }

    fn adjacent_numbers_at(&self, row: i32, col: i32) -> Vec<u32> {
        self.numbers
            .iter()
            .fold(Vec::new(), |mut acc, &((i, j), n)| {
                let col_start = j as i32 - n.to_string().len() as i32;
                let col_end = j as i32;
                let in_range = i as i32 == row && col_start < col && col <= col_end;

                if in_range {
                    acc.push(n);
                }

                acc
            })
    }

    fn numbers(matrix: &CharMatrix) -> Vec<((usize, usize), u32)> {
        matrix
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
            .collect()
    }

    fn gear_candidates(matrix: &CharMatrix) -> Vec<(usize, usize)> {
        matrix
            .iter()
            .enumerate()
            .flat_map(|(i, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .filter(|(_, &c)| Self::is_gear_marker(c))
                    .map(move |(j, _)| (i, j))
            })
            .collect()
    }

    #[inline]
    fn is_gear_marker(c: char) -> bool {
        c == '*'
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let engine_schematic = EngineSchematic::new(input);
    let result = engine_schematic.calculate_gear_ratio_sum();
    Ok(result.to_string())
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
