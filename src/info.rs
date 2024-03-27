//! General information about the crate and the emulator.

#[cfg(not(feature="no-buildinfo"))]
use crate::{
    gen::{COMPILATION_DATE, COMPILATION_TIME, COMPILER, COMPILER_VERSION, NAME, VERSION},
    util::capitalize,
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Info;

#[cfg(not(feature="no-buildinfo"))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Info {
    /// Obtains the name of the emulator.
    pub fn name() -> String {
        capitalize(NAME)
    }

    /// Obtains the version of the emulator.
    pub fn version() -> String {
        String::from(VERSION)
    }

    /// Obtains the system this emulator is emulating.
    pub fn system() -> String {
        String::from("Game Boy")
    }

    /// Obtains the name of the compiler that has been
    /// used in the compilation of the base Boytacean
    /// library. Can be used for diagnostics.
    pub fn compiler() -> String {
        String::from(COMPILER)
    }

    pub fn compiler_version() -> String {
        String::from(COMPILER_VERSION)
    }

    pub fn compilation_date() -> String {
        String::from(COMPILATION_DATE)
    }

    pub fn compilation_time() -> String {
        String::from(COMPILATION_TIME)
    }
}

#[cfg(feature="no-buildinfo")]
impl Info {
    pub fn name() -> String {
        String::from("Boytacean")
    }
    pub fn version() -> String {
        String::from("git")
    }
    pub fn system() -> String {
        String::from("Game Boy")
    }
    pub fn compiler() -> String {
        String::from("Some compiler")
    }
    pub fn compiler_version() -> String {
        String::from("some compiler version")
    }
    pub fn compilation_date() -> String {
        String::from("some date")
    }
    pub fn compilation_time() -> String {
        String::from("some time")
    }
}
