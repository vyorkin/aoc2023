#!/usr/bin/env rust-script

//! ```cargo
//! [package]
//! edition = "2021"
//!
//! [dependencies]
//! clap = { version = "4.4", features = ["derive"] }
//! nom = "7.1"
//! reqwest = { version = "0.11", features = ["blocking"] }
//! dotenvy = "0.15"
//! ```

use clap::{error::ErrorKind, CommandFactory, Parser};
use dotenvy::dotenv;
use nom::{bytes::complete::tag, character::complete, sequence::preceded, IResult};
use reqwest::{blocking::Client, header::COOKIE};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// Expected to have format like "day-xx"
    #[clap(short, long)]
    day: String,

    /// For justfile
    #[clap(long("current-working-directory"))]
    cwd: PathBuf,
}

fn parse_day(input: &str) -> IResult<&str, u32> {
    preceded(tag("day-"), complete::u32)(input)
}

fn main() -> Result<(), reqwest::Error> {
    dotenv().expect(".env file not found");
    let session = std::env::var("SESSION").expect("SESSION environment variable is not set");
    let args = Args::parse();

    let Ok((_, day)) = parse_day(&args.day) else {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("day `{}` must be formatted as `day-xx`", args.day),
        )
        .exit();
    };

    let url = format!("https://adventofcode.com/2023/day/{day}/input");
    println!("sending to `{}`", url);

    let client = Client::new();
    let input_data = client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()?
        .text()?;

    for filename in ["input1.txt", "input2.txt"] {
        let file_path = args.cwd.join(&args.day).join(filename);
        let mut file = File::create(&file_path).expect("unable to create file");
        file.write_all(input_data.as_bytes())
            .expect("unable to write input file");
        println!("wrote {}", file_path.display());
    }

    Ok(())
}
