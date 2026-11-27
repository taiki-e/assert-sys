// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(target_os = "openbsd")]
#![allow(dead_code, non_camel_case_types)]

use std::os::raw::c_long;

#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: c_long,
}

extern "C" {
    fn futex(addr: *mut u32, op: i32, val: i32, ts: *const timespec, addr2: *mut u32) -> i32;
}

assert_sys::static_assert_sys_fn! {
    struct timespec {
        tv_sec: i64,
        tv_nsec: c_long,
    }
    extern "C" {
        fn futex(addr: *mut u32, op: i32, val: i32, ts: *const timespec, addr2: *mut u32) -> i32;
    }
}
