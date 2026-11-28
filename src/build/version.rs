// SPDX-License-Identifier: Apache-2.0 OR MIT

// rustc version parsing used in the build script.
//
// Adapted from https://github.com/taiki-e/atomic-maybe-uninit

use std::{env, iter, process::Command, str};

pub(crate) fn rustc_version() -> Option<Version> {
    let rustc = env::var_os("RUSTC")?;
    let rustc_wrapper = env::var_os("RUSTC_WRAPPER").filter(|v| !v.is_empty());
    // Do not apply RUSTC_WORKSPACE_WRAPPER: https://github.com/cuviper/autocfg/issues/58#issuecomment-2067625980
    let mut rustc = rustc_wrapper.into_iter().chain(iter::once(rustc));
    let mut cmd = Command::new(rustc.next().unwrap());
    cmd.args(rustc);
    // Use verbose version output because the packagers add extra strings to the normal version output.
    // Do not use long flags (--version --verbose) because clippy-deriver doesn't handle them properly.
    // -vV is also matched with that cargo internally uses: https://github.com/rust-lang/cargo/blob/0.80.0/src/cargo/util/rustc.rs#L65
    let output = cmd.arg("-vV").output().ok()?;
    let verbose_version = str::from_utf8(&output.stdout).ok()?;
    Version::parse(verbose_version)
}

pub(crate) struct Version {
    pub(crate) minor: u32,
    commit_date: Option<Date>,
}

impl Version {
    // The known latest stable version. If we unable to determine
    // the rustc version, we assume this is the current version.
    // It is no problem if this is older than the actual latest stable.
    pub(crate) const LATEST: Self = Self::stable(100);

    const fn stable(rustc_minor: u32) -> Self {
        Self { minor: rustc_minor, commit_date: None }
    }

    pub(crate) fn probe(&self, minor: u32, year: u16, month: u8, day: u8) -> bool {
        if self.minor != minor {
            return self.minor > minor;
        }
        if let Some(commit_date) = &self.commit_date {
            *commit_date >= Date::new(year, month, day)
        } else {
            true
        }
    }

    fn parse(verbose_version: &str) -> Option<Self> {
        let mut release = verbose_version
            .lines()
            .find(|line| line.starts_with("release: "))
            .map(|line| &line["release: ".len()..])?
            .splitn(2, '-');
        let version = release.next().unwrap();
        let channel = release.next().unwrap_or_default();
        let mut digits = version.splitn(3, '.');
        let major = digits.next()?;
        if major != "1" {
            return None;
        }
        let minor = digits.next()?.parse::<u32>().ok()?;
        let _patch = digits.next().unwrap_or("0").parse::<u32>().ok()?;
        let nightly = channel == "nightly" || channel == "dev";
        // We don't refer commit date on stable/beta, but refer it even if RUSTC_BOOTSTRAP=-1 is set.
        let refer_commit_date = nightly;

        if refer_commit_date {
            let commit_date = (|| {
                let mut commit_date = verbose_version
                    .lines()
                    .find(|line| line.starts_with("commit-date: "))
                    .map(|line| &line["commit-date: ".len()..])?
                    .splitn(3, '-');
                let year = commit_date.next()?.parse::<u16>().ok()?;
                let month = commit_date.next()?.parse::<u8>().ok()?;
                let day = commit_date.next()?.parse::<u8>().ok()?;
                if month > 12 || day > 31 {
                    return None;
                }
                Some(Date::new(year, month, day))
            })();
            Some(Self { minor, commit_date: Some(commit_date.unwrap_or(Date::UNKNOWN)) })
        } else {
            Some(Self::stable(minor))
        }
    }
}

#[derive(PartialEq, PartialOrd)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    const UNKNOWN: Self = Self::new(0, 0, 0);

    const fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}
