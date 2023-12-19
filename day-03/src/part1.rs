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
                .map(|(row_ix, col_ix, cell)| ((*row_ix, *col_ix), *cell))
        })
        .collect::<BTreeMap<(usize, usize), Cell>>();

    let cells_digits = cells_tuples
        .iter()
        .map(|row| {
            row.iter()
                .filter_map(|&(row_ix, col_ix, cell)| {
                    if let Cell::Digit(n) = cell {
                        Some((row_ix, col_ix, n))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<(usize, usize, u32)>>>();

    // (-1, -1) (-1, 0) (-1, 1)
    // ( 0, -1)  xxxxx  ( 0, 1)
    // ( 1, -1) ( 1, 0) ( 1, 1)
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

    let adjacent_symbols = cells_digits
        .iter()
        .flat_map(|row| {
            row.iter()
                .map(|&(row_ix, col_ix, _)| {
                    let adjacent = offsets
                        .iter()
                        .filter_map(|&(row_dx, col_dx)| {
                            let row_ix = row_ix as i32 + row_dx;
                            let col_ix = col_ix as i32 + col_dx;
                            if row_ix >= 0 && col_ix >= 0 {
                                cells_map
                                    .get(&(row_ix as usize, col_ix as usize))
                                    .copied()
                                    .filter(Cell::is_symbol)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    ((row_ix, col_ix), adjacent)
                })
                .collect::<Vec<_>>()
        })
        .collect::<BTreeMap<(usize, usize), Vec<Cell>>>();

    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..

    let sum = cells_digits
        .into_iter()
        .flat_map(|row| {
            row.into_iter()
                .filter_map(|(row_ix, col_ix, n)| {
                    let key = &(row_ix, col_ix);
                    if adjacent_symbols.contains_key(key) {
                        Some(n)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .sum::<u32>();

    Ok(sum.to_string())
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
