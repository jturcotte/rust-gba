#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![warn(missing_docs)]

//! This crate helps you write GBA ROMs.
//!
//! # SAFETY POLICY
//!
//! Some parts of this crate are safe wrappers around unsafe operations. This is
//! good, and what you'd expect from a Rust crate.
//!
//! However, the safe wrappers all assume that you will _only_ attempt to
//! execute this crate on a GBA or in a GBA Emulator.
//!
//! **Do not** use this crate in programs that aren't running on the GBA. If you
//! do, it's a giant bag of Undefined Behavior.
//!
//! # TESTING POLICY
//!
//! It is the intent of the crate authors that as much of the crate as possible
//! be written so that you can use `cargo test` for at least some parts of your
//! code without everything exploding instantly. To that end, where possible we
//! attempt to use `cfg` flags to make things safe for `cargo test`. Hopefully
//! we got it all.

pub mod core_extras;
pub(crate) use crate::core_extras::*;

pub mod io_registers;

pub mod video_ram;
pub(crate) use crate::video_ram::*;

pub mod intrinsics;

/// Combines the Red, Blue, and Green provided into a single color value.
pub const fn rgb16(red: u16, green: u16, blue: u16) -> u16 {
  blue << 10 | green << 5 | red
}

/// BIOS Call: Div (GBA SWI 0x06).
///
/// Gives the DIV and MOD output of `numerator / denominator`.
///
/// # Panics
///
/// If `denominator` is 0.
#[inline]
pub fn div_mod(numerator: i32, denominator: i32) -> (i32, i32) {
  // The BIOS includes several System Call Functions which can be accessed by
  // SWI instructions. Incoming parameters are usually passed through registers
  // R0,R1,R2,R3. Outgoing registers R0,R1,R3 are typically containing either
  // garbage, or return value(s). All other registers (R2,R4-R14) are kept
  // unchanged. --GBATEK
  assert!(denominator != 0);
  let div_out: i32;
  let mod_out: i32;
  unsafe {
    asm!(/* assembly template */ "swi 0x06"
        :/* output operands */ "={r0}"(div_out), "={r1}"(mod_out)
        :/* input operands */ "{r0}"(numerator), "{r1}"(denominator)
        :/* clobbers */ "r3"
        :/* options */
    );
  }
  (div_out, mod_out)
}
