#![deny(clippy::expect_used)]

pub fn forbidden_hidden_panic(input: &str) -> u64 {
    input.parse::<u64>().expect("negative Clippy fixture")
}

pub fn forbidden_debug_macro(value: u64) -> u64 {
    dbg!(value)
}
