use crate::error::AocError;

fn is_symbol(char: char) -> bool {
    !char.is_ascii_digit() && char != '.'
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let matrix = input
        .lines()
        .map(|line| format!("{line}.").chars().collect())
        .collect::<Vec<Vec<char>>>();

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
        .collect::<Vec<((usize, usize), u32)>>();

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

    let sum = numbers
        .into_iter()
        .filter(|&((i, j), n)| {
            offsets.iter().any(|&(row_offset, col_offset)| {
                let num_digits = n.to_string().len();
                [0, num_digits - 1].into_iter().any(|d| {
                    let row = i as i32 + row_offset;
                    let col = j as i32 + col_offset - d as i32;
                    matrix
                        .get(row as usize)
                        .and_then(|row| row.get(col as usize))
                        .map(|&c| is_symbol(c))
                        .unwrap_or(false)
                })
            })
        })
        .map(|x| x.1)
        .sum::<u32>();

    Ok(sum.to_string())
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        assert_eq!("925", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56";
        assert_eq!("413", process(input)?);
        Ok(())
    }
}
