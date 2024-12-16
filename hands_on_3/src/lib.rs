use std::{fmt::Debug, str::FromStr};

pub fn parse_line<T: FromStr<Err: Debug>>(line: &str) -> Vec<T> {
    line.split_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<_>>()
}
