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

#[cfg(test)]
extern crate std;

// rustfmt-compatible cfg_select/cfg_if alternative
// Note: This macro is cfg_sel!({ }), not cfg_sel! { }.
// An extra brace is used in input to make contents rustfmt-able.
#[allow(unused_macros)]
macro_rules! cfg_sel {
    ({#[cfg(else)] { $($output:tt)* }}) => {
        $($output)*
    };
    ({
        #[cfg($cfg:meta)]
        { $($output:tt)* }
        $($( $rest:tt )+)?
    }) => {
        #[cfg($cfg)]
        cfg_sel! {{#[cfg(else)] { $($output)* }}}
        $(
            #[cfg(not($cfg))]
            cfg_sel! {{ $($rest)+ }}
        )?
    };
}

#[path = "gen/sys/mod.rs"]
pub mod sys;

// Not public API.
#[doc(hidden)]
pub mod __private {
    pub use core::{
        mem::{self, align_of, size_of, transmute},
        primitive::{f64, u128, usize},
        ptr::addr_of,
    };

    pub use crate::const_cmp::{Scalar, Value};
}

/// Static assertions for FFI bindings.
///
/// This checks that FFI bindings defined in user code and FFI bindings generated for
/// the platform have the same types.
/// Since this is static assertion, we can detect problems with
/// `cargo check --tests --target <target>` run in CI
/// without actually running tests on these platforms.
#[macro_export]
macro_rules! static_assert_sys_type {
    ($(
        $(#[$attr:meta])*
        type $name:ident;
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
/// This checks that FFI bindings defined in user code and FFI bindings generated for
/// the platform have the same structs.
/// Since this is static assertion, we can detect problems with
/// `cargo check --tests --target <target>` run in CI
/// without actually running tests on these platforms.
#[macro_export]
macro_rules! static_assert_sys_struct {
    ($(
        $(#[$attr:meta])*
        struct $name:ident {$(
            $(#[$field_attr:meta])*
            $field_name:ident: $field_ty:ty,
        )*}
    )*) => {
        #[allow(unused_imports)]
        const _: fn() = || {
            $crate::__static_assert_sys_import!();
            $(
                $(#[$attr])*
                {
                    $crate::__static_assert!(
                        $crate::__private::size_of::<$name>()
                            == $crate::__private::size_of::<sys::$name>()
                    );
                    $crate::__static_assert!(
                        $crate::__private::align_of::<$name>()
                            == $crate::__private::align_of::<sys::$name>()
                    );
                    let _ = |local: &$name, reference: &sys::$name| {
                        // Check missing or extra field.
                        let $name { $($(#[$field_attr])* $field_name: _,)* } = local;
                        let sys::$name { $($(#[$field_attr])* $field_name: _,)* } = reference;
                        $(
                            $(#[$field_attr])*
                            {
                                // The outer &mut makes the pointee type invariant, preventing implicit field coercions.
                                let mut local_ptr = $crate::__private::addr_of!(local.$field_name);
                                let mut reference_ptr =
                                    $crate::__private::addr_of!(reference.$field_name);
                                let _: [&mut *const $field_ty; 2] =
                                    [&mut local_ptr, &mut reference_ptr];
                                $crate::__static_assert_sys_struct_offset_cmp!($name, $field_name);
                            }
                        )*
                    };
                }
            )*
        };
    };
}
#[cfg(not(assert_sys_no_mem_offset_of))]
#[macro_export]
#[doc(hidden)] // Not public API.
macro_rules! __static_assert_sys_struct_offset_cmp {
    ($name:ident, $field_name:ident) => {
        $crate::__static_assert!(
            $crate::__private::mem::offset_of!($name, $field_name)
                == $crate::__private::mem::offset_of!(sys::$name, $field_name),
        );
    };
}
#[cfg(assert_sys_no_mem_offset_of)]
#[macro_export]
#[doc(hidden)] // Not public API.
macro_rules! __static_assert_sys_struct_offset_cmp {
    ($name:ident, $field_name:ident) => {};
}

/// Static assertions for FFI bindings.
///
/// This checks that FFI bindings defined in user code and FFI bindings generated for
/// the platform have the same values.
/// Since this is static assertion, we can detect problems with
/// `cargo check --tests --target <target>` run in CI
/// without actually running tests on these platforms.
#[macro_export]
macro_rules! static_assert_sys_const {
    ($(
        $(#[$attr:meta])*
        const $name:ident: $ty:ty;
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
                    $crate::__static_assert_sys_const_cmp!($name, $ty);
                }
            )*
        };
    };
}
#[macro_export]
#[doc(hidden)] // Not public API.
macro_rules! __static_assert_sys_const_cmp {
    ($name:ident, $ty:ty) => {
        $crate::__static_assert!({
            let values = (
                $crate::__private::Scalar::<$ty>($name).value(),
                $crate::__private::Scalar(sys::$name).value(),
            );
            // Comparison code is inlined because float comparison in const fn require Rust 1.82.
            #[allow(clippy::float_arithmetic)]
            {
                use $crate::__private::{Value, f64, u128, usize};
                match values {
                    (Value::Int(a_negative, a), Value::Int(b_negative, b)) => {
                        a_negative == b_negative && a == b
                    }
                    (Value::Float(a), Value::Float(b)) => a == b,
                    (Value::Int(negative, magnitude), Value::Float(float))
                    | (Value::Float(float), Value::Int(negative, magnitude)) => {
                        let abs = if negative { -float } else { float };
                        // u128::MAX rounds up to 2^128 in f64. Exclude that boundary
                        // before casting, and rule out rounding in both directions.
                        abs < u128::MAX as f64
                            && abs as u128 == magnitude
                            && magnitude as f64 == abs
                    }
                    (Value::Ptr(a), Value::Ptr(b)) => {
                        // ptr comparison and ptr-to-int cast are not stable on const context.
                        // SAFETY: Null and integer-derived FFI sentinels have no allocation
                        // provenance. Const evaluation rejects allocation-backed addresses.
                        unsafe {
                            $crate::__private::transmute::<*const (), usize>(a)
                                == $crate::__private::transmute::<*const (), usize>(b)
                        }
                    }
                    _ => false,
                }
            }
        });
    };
}

mod const_cmp {
    pub struct Scalar<T>(pub T);

    #[must_use]
    pub enum Value {
        Int(/* sign */ bool, /* magnitude */ u128),
        Float(f64),
        Ptr(*const ()),
    }

    macro_rules! scalar {
        ($($ty:ty),+ => |$v:ident| $cast:expr) => {$(
            impl Scalar<$ty> {
                #[allow(clippy::unnecessary_cast)]
                pub const fn value(self) -> Value {
                    let $v = self.0;
                    $cast
                }
            }
        )+};
    }
    scalar!(u8, u16, u32, u64, u128, usize, bool, char => |v| Value::Int(false, v as u128));
    scalar!(i8, i16, i32, i64, i128, isize => |v| Value::Int(v < 0, v.unsigned_abs() as u128));
    scalar!(f32, f64 => |v| Value::Float(v as f64));

    impl<T> Scalar<*const T> {
        pub const fn value(self) -> Value {
            Value::Ptr(self.0.cast::<()>())
        }
    }
    impl<T> Scalar<*mut T> {
        pub const fn value(self) -> Value {
            Value::Ptr(self.0.cast::<()>())
        }
    }
}

/// Static assertions for FFI bindings.
///
/// This checks that FFI bindings defined in user code and FFI bindings generated for
/// the platform have the same signatures.
/// Since this is static assertion, we can detect problems with
/// `cargo check --tests --target <target>` run in CI
/// without actually running tests on these platforms.
#[macro_export]
macro_rules! static_assert_sys_fn {
    (
        $(#[$extern_attr:meta])*
        extern $abi:literal {$(
            $(#[$fn_attr:meta])*
            fn $name:ident($($args:tt)*) $(-> $ret_ty:ty)?;
        )*}
    ) => {
        #[allow(unused_imports)]
        const _: fn() = || {
            $crate::__static_assert_sys_import!();
            $(
                $(#[$fn_attr])*
                {
                    $crate::__static_assert_sys_fn_cmp!($abi fn $name($($args)*) $(-> $ret_ty)?);
                }
            )*
        };
    };
}
#[macro_export]
#[doc(hidden)] // Not public API.
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
#[doc(hidden)] // Not public API.
macro_rules! __static_assert_sys_import {
    () => {
        use ::libc as sys;
    };
}
#[cfg(target_os = "redox")]
#[macro_export]
#[doc(hidden)] // Not public API.
macro_rules! __static_assert_sys_import {
    () => {
        use ::syscall as sys;
    };
}
#[cfg(not(any(
    target_os = "aix",
    target_os = "hermit",
    target_os = "nto",
    target_os = "qnx",
    target_os = "redox",
)))]
#[macro_export]
#[doc(hidden)] // Not public API.
macro_rules! __static_assert_sys_import {
    () => {
        use $crate::sys;
    };
}

// Do not use inline_const-based __static_assert! because it is not always evaluated with `cargo check`
// (`cargo build` and `cargo test` are fine).
#[macro_export]
#[doc(hidden)] // Not public API.
macro_rules! __static_assert {
    ($cond:expr $(,)?) => {{
        const _: [(); 1] = [(); if $cond { 1 } else { 0 }];
    }};
}
