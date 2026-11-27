// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(all(
    target_os = "linux",
    target_arch = "x86_64",
    target_pointer_width = "64",
    target_env = "gnu"
))]
#![allow(dead_code, non_camel_case_types)]

#[repr(C)]
struct robust_list {
    next: *mut robust_list,
}

#[repr(C)]
struct robust_list_head {
    list: robust_list,
    futex_offset: i64,
    list_op_pending: *mut robust_list,
}

assert_sys::static_assert_sys_struct! {
    struct robust_list { next: *mut _, }
    struct robust_list_head {
        list: _ {
            next: _,
        },
        futex_offset: _,
        list_op_pending: _,
    }
}
