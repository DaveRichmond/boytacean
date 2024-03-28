#![cfg_attr(feature = "embedded", no_std)]
#![allow(clippy::uninlined_format_args)]

#[cfg(feature = "embedded")]
extern crate alloc;

pub mod apu;
pub mod cheats;
pub mod consts;
pub mod cpu;
pub mod data;
pub mod devices;
pub mod dma;
pub mod error;
pub mod gb;
pub mod gen;
pub mod info;
pub mod inst;
pub mod macros;
pub mod mmu;
pub mod pad;
pub mod ppu;
pub mod rom;
pub mod serial;
#[cfg(not(feature = "embedded"))]
pub mod state;
#[cfg(not(feature = "embedded"))]
pub mod test;
pub mod timer;
pub mod util;

#[cfg(feature = "python")]
pub mod py;
