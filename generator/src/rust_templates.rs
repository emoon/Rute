pub static HEADER: &'static [u8] = b"
// ***************************************************************
// AUTO-GENERATED! DO NOT EDIT!
// ***************************************************************

use ffi_gen::*;
pub use ffi_gen::PUBase as PUBase;\n\n
use std::ffi::CString;\n\n";

pub static UI_HEADER: &'static [u8] = b"pub struct Ui {
    pu: *const PU
}

impl Ui {
    pub fn new(pu: *const PU) -> Ui { Ui { pu: pu } }
\n";



