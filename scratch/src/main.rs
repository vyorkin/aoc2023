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
    use itertools::unfold;

    #[test]
    fn test_unfold_digit_names() {
        let input = "xtwone3four";
        let digits = unfold(String::from(input), |s| {
            let (result, skip) = if s.starts_with("one") {
                (Some('1'), 3)
            } else if s.starts_with("two") {
                (Some('2'), 3)
            } else if s.starts_with("three") {
                (Some('3'), 5)
            } else if s.starts_with("four") {
                (Some('4'), 4)
            } else if s.starts_with("five") {
                (Some('5'), 4)
            } else if s.starts_with("six") {
                (Some('6'), 3)
            } else if s.starts_with("seven") {
                (Some('7'), 5)
            } else if s.starts_with("eight") {
                (Some('8'), 5)
            } else if s.starts_with("nine") {
                (Some('9'), 4)
            } else if let Some((_, c)) = s
                .chars()
                .next()
                .and_then(|c| c.to_digit(10).map(|d| (d, c)))
            {
                (Some(c), 1)
            } else {
                (None, 1)
            };

            *s = String::from(&s[skip..]);

            result
        });
        let result = digits.take(3).collect::<Vec<_>>();
        assert_eq!(result, vec!['a']);
    }

    #[test]
    fn test_unfold_konyacci() {
        let konyacci = unfold((String::from("x"), String::from("y")), |(s1, s2)| {
            let ss1 = format!("{s1}{s2}");
            let ss2 = format!("{s2}{s1}");
            *s1 = ss1.clone();
            *s2 = ss2;
            let next = ss1;
            Some(next)
        });
        // itertools::assert_equal(konyacci.by_ref().take(6), vec!["xy"]);
        assert_eq!(
            konyacci.skip(2).take(3).collect::<Vec<String>>(),
            vec![
                "xyyxyxxy",
                "xyyxyxxyyxxyxyyx",
                "xyyxyxxyyxxyxyyxyxxyxyyxxyyxyxxy"
            ]
        )
    }

    #[test]
    fn test_unfold_fibonacci() {
        let mut fibonacci = unfold((1u32, 1u32), |(x1, x2)| {
            // Attempt to get the next Fibonacci number
            let next = x1.saturating_add(*x2);

            // Shift left: ret <- x1 <- x2 <- next
            let ret = *x1;
            *x1 = *x2;
            *x2 = next;

            // If addition has saturated at the maximum, we are finished
            if ret == *x1 && ret > 1 {
                None
            } else {
                Some(ret)
            }
        });

        itertools::assert_equal(fibonacci.by_ref().take(8), vec![1, 1, 2, 3, 5, 8, 13, 21]);
        assert_eq!(fibonacci.last(), Some(2_971_215_073))
    }
}
