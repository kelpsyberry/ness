#![feature(drain_filter, adt_const_params, array_chunks)]
#![allow(incomplete_features)]

use serde::{Deserialize, Serialize};

pub extern crate emu_utils as utils;

pub mod cart;
pub mod cpu;
pub mod emu;
pub mod ppu;
pub mod schedule;
mod wram;
pub use wram::Wram;

bitflags::bitflags! {
    pub struct Keys: u16 {
        const R = 1 << 4;
        const L = 1 << 5;
        const X = 1 << 6;
        const A = 1 << 7;
        const RIGHT = 1 << 8;
        const LEFT = 1 << 9;
        const DOWN = 1 << 10;
        const UP = 1 << 11;
        const START = 1 << 12;
        const SELECT = 1 << 13;
        const Y = 1 << 14;
        const B = 1 << 15;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    Ntsc,
    Pal,
}
