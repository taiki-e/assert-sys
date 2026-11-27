// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(dead_code, non_camel_case_types)]

#[cfg(all(
    target_os = "linux",
    target_arch = "x86_64",
    target_pointer_width = "64",
    target_env = "gnu"
))]
mod linux {
    use assert_sys::sys::{robust_list, robust_list_head};

    assert_sys::static_assert_sys_struct! {
        struct robust_list_head {
            list: robust_list {
                #[cfg(any())]
                missing: u32,
                next: *mut robust_list,
            },
            futex_offset: i64,
            list_op_pending: *mut robust_list,
        }
    }
}

#[cfg(all(target_os = "freebsd", target_arch = "x86_64"))]
mod freebsd {
    use core::ffi::c_void;

    #[repr(C)]
    struct Elf64_Auxinfo {
        a_type: i64,
        a_un: Members,
    }
    #[repr(C)]
    union Members {
        a_val: i64,
        a_ptr: *mut c_void,
        a_fcn: Option<unsafe extern "C" fn()>,
    }

    assert_sys::static_assert_sys_struct! {
        struct Elf64_Auxinfo {
            a_type: i64,
            a_un: _ {
                a_fcn: Option<unsafe extern "C" fn()>,
                #[cfg(any())]
                a_val: u32,
                a_ptr: *mut c_void,
                a_val: i64,
            },
        }
    }
}
