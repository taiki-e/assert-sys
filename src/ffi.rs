// SPDX-License-Identifier: Apache-2.0 OR MIT

// core::ffi::* (except c_void) requires Rust 1.64

#![allow(dead_code, non_camel_case_types, unused_imports)]

pub(super) use core::ffi::c_void;

pub(super) use super::c_char;

// https://github.com/rust-lang/rust/blob/1.100.0/library/core/src/ffi/primitives.rs
pub(super) type c_uchar = u8;
pub(super) type c_ushort = u16;
pub(super) type c_longlong = i64;
pub(super) type c_ulonglong = u64;

// c_{,u}int is {i,u}16 on 16-bit targets, otherwise {i,u}32.
// https://github.com/rust-lang/rust/blob/1.100.0/library/core/src/ffi/primitives.rs#L177
cfg_sel!({
    #[cfg(target_pointer_width = "16")]
    {
        pub(super) type c_int = i16;
        pub(super) type c_uint = u16;
    }
    #[cfg(else)]
    {
        pub(super) type c_int = i32;
        pub(super) type c_uint = u32;
    }
});

// c_{,u}long is {i,u}64 on non-Windows 64-bit targets, otherwise {i,u}32.
// https://github.com/rust-lang/rust/blob/1.100.0/library/core/src/ffi/primitives.rs#L138
// Do not include WASM Linux because it is not upstreamed and ABI is different between implementations.
// https://github.com/Wasm-Thin-Kernel-Interfaces/wali-musl-libc/blob/ac08acb01cb951c725f7d05c5693a47129f5f679/arch/wasm32/bits/alltypes.h.in#L7
// https://github.com/joelseverin/linux-wasm/blob/970e08b0d605e63e36a99d22596e8899a1dab8b5/patches/musl/0001-NOMERGE-Hacks-to-get-Linux-Wasm-to-compile-minimal-a.patch#L150
#[cfg(not(all(any(target_arch = "wasm32", target_arch = "wasm64"), target_os = "linux")))]
cfg_sel!({
    #[cfg(all(target_pointer_width = "64", not(windows)))]
    {
        pub(super) type c_long = i64;
        pub(super) type c_ulong = u64;
    }
    #[cfg(else)]
    {
        pub(super) type c_long = i32;
        pub(super) type c_ulong = u32;
    }
});

// Static assertions for C type definitions.
#[cfg(test)]
const _: fn() = || {
    let _: c_uchar = 0 as std::os::raw::c_uchar;
    let _: c_ushort = 0 as std::os::raw::c_ushort;
    let _: c_longlong = 0 as std::os::raw::c_longlong;
    let _: c_ulonglong = 0 as std::os::raw::c_ulonglong;
    let _: c_int = 0 as std::os::raw::c_int;
    let _: c_uint = 0 as std::os::raw::c_uint;
    let _: c_long = 0 as std::os::raw::c_long;
    let _: c_ulong = 0 as std::os::raw::c_ulong;
};
