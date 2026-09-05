#![forbid(unsafe_code)]

use std::process::ExitCode;
use verified_logic::bounded_modular_tree_fold::{
    SEQUENTIAL_U24_ORACLE_SYMBOL, sequential_u24_sum,
};

const LEAF_COUNT: usize = 4;
const PROGRAM_ARGUMENT_COUNT: usize = LEAF_COUNT + 1;
const LEAF_LIMIT: u64 = 4;
const EXPECTED_ORACLE_SYMBOL: &str =
    "verified_logic::bounded_modular_tree_fold::sequential_u24_sum";

fn main() -> ExitCode {
    let arguments = std::env::args().collect::<Vec<_>>();
    if arguments.len() != PROGRAM_ARGUMENT_COUNT {
        eprintln!("oracle requires exactly four u24 leaf arguments");
        return ExitCode::FAILURE;
    }
    if SEQUENTIAL_U24_ORACLE_SYMBOL != EXPECTED_ORACLE_SYMBOL {
        eprintln!("oracle symbol differs from the reviewed Trellis profile");
        return ExitCode::FAILURE;
    }
    let mut leaves = [0_u64; LEAF_COUNT];
    for (index, text) in arguments.iter().skip(1).enumerate() {
        let Ok(value) = text.parse::<u64>() else {
            eprintln!("oracle leaf is not an unsigned integer");
            return ExitCode::FAILURE;
        };
        leaves[index] = value;
    }
    let Ok(outcome) = sequential_u24_sum(&leaves, LEAF_LIMIT) else {
        eprintln!("Trellis oracle rejected the supplied leaf profile");
        return ExitCode::FAILURE;
    };
    println!("Result: {}", outcome.value);
    println!("- COMBINES: {}", outcome.combine_count);
    ExitCode::SUCCESS
}
