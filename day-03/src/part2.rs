use std::collections::BTreeMap;

use itertools::Itertools;

use crate::error::AocError;

#[inline]
fn is_gear(c: char) -> bool {
    c == '*'
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let matrix = input
        .lines()
        .map(|line| format!(".{line}.").chars().collect())
        .collect::<Vec<Vec<char>>>();

    let symbols = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, cells)| {
            cells
                .iter()
                .enumerate()
                .filter_map(|(j, &c)| if is_gear(c) { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<(usize, usize)>>();

    let numbers = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, cells)| {
            cells
                .iter()
                .enumerate()
                .fold((Vec::new(), None), |(mut acc, maybe_digit), (j, char)| {
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
        .collect::<BTreeMap<(usize, usize), u32>>();

    // dbg!(numbers.clone());

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

    let gears = symbols
        .into_iter()
        .filter_map(|(i, j)| {
            let r = offsets
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
        .collect::<Vec<u32>>();

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
