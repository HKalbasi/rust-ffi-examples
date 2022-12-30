#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// bindgen failed to generate some defines, so we will manually add them here
pub const FT_ANY_COLUMN: usize = u32::MAX as usize;
pub const FT_ROW_HEADER: ::std::os::raw::c_int = 1;
