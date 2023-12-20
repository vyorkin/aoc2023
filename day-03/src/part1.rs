#![allow(dead_code, unused_variables)]

use std::collections::BTreeMap;

use crate::error::AocError;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    Dot,
    Digit(u32),
    Symbol(char),
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        if value.is_ascii_digit() {
            Cell::Digit(value.to_digit(10).expect("not a digit"))
        } else if value == '.' {
            Cell::Dot
        } else {
            Cell::Symbol(value)
        }
    }
}

impl Cell {
    pub fn is_digit(&self) -> bool {
        matches!(self, Cell::Digit(_))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Cell::Symbol(_))
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let cells_tuples = input
        .lines()
        .enumerate()
        .clone()
        .map(|(row_ix, row)| {
            row.chars()
                .enumerate()
                .map(|(col_ix, c)| (row_ix, col_ix, c.into()))
                .collect()
        })
        .collect::<Vec<Vec<(usize, usize, Cell)>>>();

    let cells_map = cells_tuples
        .iter()
        .flat_map(|row| {
            row.iter()
                .copied()
                .map(|(row_ix, col_ix, cell)| ((row_ix, col_ix), cell))
        })
        .collect::<BTreeMap<(usize, usize), Cell>>();

    let numbers_tuples = input
        .lines()
        .enumerate()
        .flat_map(|(row_ix, line)| {
            line.chars()
                .enumerate()
                .fold((Vec::new(), None), |(mut acc, current), (col_ix, ch)| {
                    if let Some(prev_digit) = current {
                        if let Some(digit) = ch.to_digit(10) {
                            let combined = prev_digit * 10 + digit;
                            (acc, Some(combined))
                        } else {
                            acc.push((row_ix, col_ix - 1, prev_digit));
                            (acc, None)
                        }
                    } else if let Some(digit) = ch.to_digit(10) {
                        (acc, Some(digit))
                    } else {
                        (acc, None)
                    }
                })
                .0
        })
        .collect::<Vec<(usize, usize, u32)>>();

    // (-1, -1) (-1, 0) (-1, 1)
    // ( 0, -1)  xxxxx  ( 0, 1)
    // ( 1, -1) ( 1, 0) ( 1, 1)

    // (-1, -2) (-1, 0) (-1, 0) (-1, 1)
    // ( 0, -2)  xxxxx   xxxxx  ( 0, 1)
    // ( 1, -2) ( 1, 0) ( 1, 0) ( 1, 1)

    let offsets: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let part_numbers = numbers_tuples
        .into_iter()
        .filter_map(|(row_ix, col_ix, n)| {
            let num_digits = n.to_string().len();

            let adjacent_symbols = offsets
                .iter()
                .filter_map(|(row_offset, col_offset)| {
                    let mut deltas = (0..num_digits).filter_map(|i| {
                        let r = row_ix as i32 + row_offset;
                        let c = col_ix as i32 + col_offset - i as i32;
                        if r >= 0 && c >= 0 {
                            Some((r as usize, c as usize))
                        } else {
                            None
                        }
                    });

                    if deltas.any(|(r, c)| {
                        cells_map
                            .get(&(r, c))
                            .copied()
                            .is_some_and(|c| c.is_symbol())
                    }) {
                        Some(n)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u32>>();

            if adjacent_symbols.is_empty() {
                None
            } else {
                Some(n)
            }
        })
        .collect::<Vec<u32>>();

    Ok(part_numbers.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
