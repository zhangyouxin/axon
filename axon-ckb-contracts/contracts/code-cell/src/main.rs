//! Generated by capsule
//!
//! `main.rs` is used to define rust lang items and modules.
//! See `entry.rs` for the `main` function.
//! See `error.rs` for the `Error` type.

#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

// define modules
#[macro_use]
mod cell;
#[macro_use]
mod common;
mod entry;
mod error;
mod pattern;

mod checker_bond_withdraw;
mod checker_join_sidechain;
mod checker_quit_sidechain;
mod checker_submit_task;

use ckb_std::{debug, default_alloc};

ckb_std::entry!(program_entry);
default_alloc!();

/// program entry
fn program_entry() -> i8 {
    // Call main function and return error code
    match entry::main() {
        Ok(_) => 0,
        Err(err) => {
            debug!("Error: {:#?}", err);
            err as i8
        }
    }
}
