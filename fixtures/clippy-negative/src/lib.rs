#![deny(clippy::expect_used)]

pub fn forbidden_hidden_panic(input: &str) -> u64 {
    input.parse::<u64>().expect("negative Clippy fixture")
}
