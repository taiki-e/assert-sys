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
mod sys_macros;
