// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

This crate contains helpers to test FFI binding definitions.

Originally extracted from test-helper from [portable-atomic](https://github.com/taiki-e/portable-atomic) repository to [test-helper](https://github.com/taiki-e/test-helper) repository.
When publishing test-helper to crates.io, a part that testing the FFI binding definitions became an issue because it had too many files despite being used only in a few repositories, so it was separated to this crate.

There are no stability guarantees for these helpers, since they're
supposed to only be used in projects managed by us.

<!-- tidy:sync-markdown-to-rustdoc:end -->
*/

#![no_std]
#![doc(test(
    no_crate_inject,
    attr(allow(
        dead_code,
        unused_variables,
        clippy::undocumented_unsafe_blocks,
        clippy::unused_trait_names,
    ))
))]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(
    // Lints that may help when writing public library.
    // missing_debug_implementations,
    // missing_docs,
    clippy::alloc_instead_of_core,
    // clippy::exhaustive_enums,
    // clippy::exhaustive_structs,
    clippy::impl_trait_in_params,
    // clippy::std_instead_of_alloc,
    // clippy::std_instead_of_core,
    // clippy::missing_inline_in_public_items,
)]

#[cfg(feature = "std")]
extern crate std;

#[path = "gen/sys/mod.rs"]
pub mod sys;

/// Static assertions for FFI bindings.
///
/// This checks that FFI bindings defined in this crate and FFI bindings generated for
/// the platform's latest header file using bindgen have the same types.
/// Since this is static assertion, we can detect problems with
/// `cargo check --tests --target <target>` run in CI
/// without actually running tests on these platforms.
/// See also tools/codegen/src/ffi.rs.
#[macro_export]
macro_rules! static_assert_sys_type {
    ($(
        $(#[$attr:meta])*
        type $([$($windows_path:ident)::+])? $name:ident;
    )*) => {
        #[allow(
            unused_imports,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation
        )]
        const _: fn() = || {
            $crate::__static_assert_sys_import!();
            $(
                $(#[$attr])*
                {
                    $(use ::windows_sys::$($windows_path)::+ as sys;)?
                    fn _check1(a: $name) -> sys::$name {
                        a
                    }
                    fn _check2(a: sys::$name) -> $name {
                        a
                    }
                }
            )*
        };
    };
}

/// Static assertions for FFI bindings.
///
/// This checks that FFI bindings defined in this crate and FFI bindings generated for
/// the platform's latest header file using bindgen have the same fields.
/// Since this is static assertion, we can detect problems with
/// `cargo check --tests --target <target>` run in CI
/// without actually running tests on these platforms.
/// See also tools/codegen/src/ffi.rs.
#[macro_export]
macro_rules! static_assert_sys_struct {
    ($(
        $(#[$attr:meta])*
        struct $([$($windows_path:ident)::+])? $name:ident {$(
            $(#[$field_attr:meta])*
            $field_name:ident: $field_ty:ty,
        )*}
    )*) => {
        #[allow(unused_imports, clippy::undocumented_unsafe_blocks)]
        const _: fn() = || {
            $crate::__static_assert_sys_import!();
            $(
                $(#[$attr])*
                {
                    $(use ::windows_sys::$($windows_path)::+ as sys;)?
                    $crate::__static_assert!(
                        ::core::mem::size_of::<$name>()
                            == ::core::mem::size_of::<sys::$name>()
                    );
                    $crate::__static_assert!(
                        ::core::mem::align_of::<$name>()
                            == ::core::mem::align_of::<sys::$name>()
                    );
                    let s1: $name = unsafe { ::core::mem::zeroed() };
                    let s2: sys::$name = unsafe { ::core::mem::zeroed() };
                    // field names and types
                    let _ = sys::$name {$(
                        $(#[$field_attr])*
                        $field_name: s1.$field_name,
                    )*};
                    let _ = $name {$(
                        $(#[$field_attr])*
                        $field_name: s2.$field_name,
                    )*};
                    // field offsets
                    $(
                        $(#[$field_attr])*
                        $crate::__static_assert_sys_struct_offset_cmp!($name, $field_name);
                    )*
                }
            )*
        };
    };
}
// mem::offset_of requires Rust 1.77
#[rustversion::since(1.77)]
#[macro_export]
#[doc(hidden)]
macro_rules! __static_assert_sys_struct_offset_cmp {
    ($name:ident, $field_name:ident) => {
        $crate::__static_assert!(
            ::core::mem::offset_of!($name, $field_name)
                == ::core::mem::offset_of!(sys::$name, $field_name),
        );
    };
}
#[rustversion::before(1.77)]
#[macro_export]
#[doc(hidden)]
macro_rules! __static_assert_sys_struct_offset_cmp {
    ($name:ident, $field_name:ident) => {};
}

/// Static assertions for FFI bindings.
///
/// This checks that FFI bindings defined in this crate and FFI bindings generated for
/// the platform's latest header file using bindgen have the same values.
/// Since this is static assertion, we can detect problems with
/// `cargo check --tests --target <target>` run in CI
/// without actually running tests on these platforms.
/// See also tools/codegen/src/ffi.rs.
#[macro_export]
macro_rules! static_assert_sys_const {
    ($(
        $(#[$attr:meta])*
        const $([$($windows_path:ident)::+])? $name:ident: $ty:ty;
    )*) => {
        #[allow(
            unused_attributes, // for #[allow(..)] in $(#[$attr])*
            unused_imports,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation,
        )]
        const _: fn() = || {
            $crate::__static_assert_sys_import!();
            $(
                $(#[$attr])*
                {
                    $(use ::windows_sys::$($windows_path)::+ as sys;)?
                    $crate::__static_assert_sys_const_cmp!($name, $ty);
                }
            )*
        };
    };
}
#[macro_export]
#[doc(hidden)]
macro_rules! __static_assert_sys_const_cmp {
    (RTLD_DEFAULT, $ty:ty) => {
        // ptr comparison and ptr-to-int cast are not stable on const context, so use ptr-to-int
        // transmute and compare its result.
        $crate::__static_assert!(
            // SAFETY: Pointer-to-integer transmutes are valid (since we are okay with losing the
            // provenance here). (Same as <pointer>::addr().)
            unsafe {
                ::core::mem::transmute::<$ty, usize>(RTLD_DEFAULT)
                    == ::core::mem::transmute::<$ty, usize>(sys::RTLD_DEFAULT)
            }
        );
    };
    ($name:ident, $ty:ty) => {
        $crate::__static_assert!(
            ($name == sys::$name as $ty) & ($name as i128 == sys::$name as i128)
        );
    };
}

/// Static assertions for FFI bindings.
///
/// This checks that FFI bindings defined in this crate and FFI bindings generated for
/// the platform's latest header file using bindgen have the same signatures.
/// Since this is static assertion, we can detect problems with
/// `cargo check --tests --target <target>` run in CI
/// without actually running tests on these platforms.
/// See also tools/codegen/src/ffi.rs.
#[macro_export]
macro_rules! static_assert_sys_fn {
    (
        $(#[$extern_attr:meta])*
        extern $abi:literal {$(
            $(#[$fn_attr:meta])*
            fn $([$($windows_path:ident)::+])? $name:ident($($args:tt)*) $(-> $ret_ty:ty)?;
        )*}
    ) => {
        #[allow(unused_imports)]
        const _: fn() = || {
            $crate::__static_assert_sys_import!();
            $(
                $(#[$fn_attr])*
                {
                    $(use ::windows_sys::$($windows_path)::+ as sys;)?
                    $crate::__static_assert_sys_fn_cmp!($abi fn $name($($args)*) $(-> $ret_ty)?);
                }
            )*
        };
    };
}
#[macro_export]
#[doc(hidden)]
macro_rules! __static_assert_sys_fn_cmp {
    (
        $abi:literal fn $name:ident($($_arg_pat:ident: $arg_ty:ty),*, ...) $(-> $ret_ty:ty)?
    ) => {
        let mut _f: unsafe extern $abi fn($($arg_ty),*, ...) $(-> $ret_ty)? = $name;
        _f = sys::$name;
    };
    (
        $abi:literal fn $name:ident($($_arg_pat:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?
    ) => {
        let mut _f: unsafe extern $abi fn($($arg_ty),*) $(-> $ret_ty)? = $name;
        _f = sys::$name;
    };
}

#[cfg(any(target_os = "aix", target_os = "hermit", target_os = "nto", target_os = "qnx"))]
#[macro_export]
macro_rules! __static_assert_sys_import {
    () => {
        use ::libc as sys;
    };
}
#[cfg(target_os = "redox")]
#[macro_export]
macro_rules! __static_assert_sys_import {
    () => {
        use ::syscall as sys;
    };
}
#[cfg(windows)]
#[macro_export]
macro_rules! __static_assert_sys_import {
    () => {};
}
#[cfg(not(any(
    target_os = "aix",
    target_os = "hermit",
    target_os = "nto",
    target_os = "qnx",
    target_os = "redox",
    windows,
)))]
#[macro_export]
macro_rules! __static_assert_sys_import {
    () => {
        use $crate::sys;
    };
}

// Do not use inline_const-based __static_assert! because it is not always evaluated with `cargo check`
// (`cargo build` and `cargo test` are fine).
#[macro_export]
#[doc(hidden)]
macro_rules! __static_assert {
    ($cond:expr $(,)?) => {{
        const _: [(); 1] = [(); (true /* type check */ & $cond) as usize];
    }};
}
