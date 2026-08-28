#![forbid(unsafe_code)]
#![allow(
    unexpected_cfgs,
    reason = "The pinned Dylint driver injects cfg(dylint_lib = \"octet\") only during Octet checks."
)]
#![cfg_attr(dylint_lib = "octet", feature(register_tool))]
#![cfg_attr(dylint_lib = "octet", register_tool(tigerstyle))]
#![cfg_attr(
    dylint_lib = "octet",
    allow(
        tigerstyle::excessive_file_length,
        tigerstyle::module_file_count,
        reason = "Octet checks structural lints at the crate root; the fixture-covered shell modules need separate decompositions."
    )
)]

#[path = "nickel/sandbox.rs"]
pub mod nickel_sandbox;
#[path = "root/io.rs"]
pub mod root_io;
#[path = "steel/runner.rs"]
pub mod steel_runner;
pub mod world_commit_shell;
