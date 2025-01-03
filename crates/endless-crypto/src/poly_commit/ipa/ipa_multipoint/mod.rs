// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

//! This module provides the poly-commit

pub mod committer;
pub mod crs;
mod default_crs;
pub mod ipa; // follows the BCMS20 scheme
pub mod math_utils;
pub mod multiproof;
pub mod transcript;

pub mod lagrange_basis;

// TODO: We use the IO Result while we do not have a dedicated Error enum
pub type IOResult<T> = std::io::Result<T>;
pub type IOError = std::io::Error;
pub type IOErrorKind = std::io::ErrorKind;
