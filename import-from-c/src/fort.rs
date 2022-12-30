//! A safe wrapper over fort's C api

/// bindgen generated bindings for fort
mod fort_c;

use std::{
    ffi::{CStr, CString},
    fmt::Display,
    ptr,
};

use fort_c::*;

pub struct FortTable {
    ptr: *mut ft_table_t,
}

impl FortTable {
    pub fn new() -> FortTable {
        // Safety: We just call a function, and panic if it fails and returns none
        let ptr = unsafe { ft_create_table() };
        if ptr.is_null() {
            panic!("Creating fort table failed");
        }
        FortTable { ptr }
    }

    pub fn add_row(&mut self, cells: &[&str]) {
        for cell in cells {
            let cell_as_cstr = CString::new(*cell).unwrap();
            unsafe {
                ft_row_write(self.ptr, 1, &mut cell_as_cstr.as_ptr());
            }
        }
        unsafe {
            ft_row_write_ln(self.ptr, 0, ptr::null_mut());
        }
    }

    pub fn add_header(&mut self, cells: &[&str]) {
        if unsafe { ft_row_count(self.ptr) } != 0 {
            panic!("Header should be the first row always");
        }
        let result = unsafe {
            ft_set_cell_prop(self.ptr, 0, FT_ANY_COLUMN, FT_CPROP_ROW_TYPE, FT_ROW_HEADER)
        };
        if result < 0 {
            panic!("set cell prop failed with error code {result}");
        }
        self.add_row(cells);
    }
}

impl Display for FortTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cstr_ptr = unsafe { ft_to_string(self.ptr) };
        let cstr = unsafe { CStr::from_ptr(cstr_ptr) };
        let rust_str = cstr.to_str().unwrap();
        write!(f, "{rust_str}")
    }
}

impl Drop for FortTable {
    fn drop(&mut self) {
        unsafe {
            ft_destroy_table(self.ptr);
        }
    }
}
