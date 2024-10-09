// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::str::FromStr;

// From this library

pub fn parse_version(version: &str) -> u64 {
    // Remove `.` separator between major, minor, patch numbers in semver string.
    let version = version.trim().replace('.', "");

    u64::from_str(version.as_str()).unwrap()
}
