//! Encoding tables for RISC-V.

use super::registers::*;
use crate::{
    ir, isa,
    isa::{
        constraints::*,
        enc_tables::*,
        encoding::{base_size, RecipeSizing},
    },
    predicates,
};

// Include the generated encoding tables:
// - `LEVEL1_RV32`
// - `LEVEL1_RV64`
// - `LEVEL2`
// - `ENCLIST`
// - `INFO`
include!(concat!(env!("OUT_DIR"), "/encoding-riscv.rs"));
include!(concat!(env!("OUT_DIR"), "/legalize-riscv.rs"));
