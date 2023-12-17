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
