// SPDX-License-Identifier: Apache-2.0 OR MIT

// The rustc-cfg emitted by the build script are *not* public API.

#[path = "src/build/version.rs"]
mod version;

use std::env;

use self::version::{Version, rustc_version};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let version = match rustc_version() {
        Some(version) => version,
        None => {
            if env::var_os("ASSERT_SYS_DENY_WARNINGS").is_some() {
                panic!("unable to determine rustc version")
            }
            println!(
                "cargo:warning={}: unable to determine rustc version; assuming latest stable rustc (1.{})",
                env!("CARGO_PKG_NAME"),
                Version::LATEST.minor
            );
            Version::LATEST
        }
    };

    if version.minor >= 80 {
        // Custom cfgs set by build script. Not public API.
        // grep -F 'cargo:rustc-cfg=' build.rs | grep -Ev '^ *//' | sed -E 's/^.*cargo:rustc-cfg=//; s/(=\\)?".*$//' | LC_ALL=C sort -u | tr '\n' ',' | sed -E 's/,$/\n/'
        println!("cargo:rustc-check-cfg=cfg(assert_sys_no_mem_offset_of)");
    }

    // Note that cfgs are `no_`*, not `has_*`. This allows treating as the latest
    // stable rustc is used when the build script doesn't run. This is useful
    // for non-cargo build systems that don't run the build script.

    // mem::offset_of requires Rust 1.77 (nightly-2024-01-21 https://github.com/rust-lang/rust/pull/118799)
    if !version.probe(77, 2024, 1, 20) {
        println!("cargo:rustc-cfg=assert_sys_no_mem_offset_of");
    }
}
