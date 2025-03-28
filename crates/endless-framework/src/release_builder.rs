// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use crate::{
    built_package::{BuildOptions, BuiltPackage},
    path_relative_to_crate,
    release_bundle::{ReleaseBundle, ReleasePackage},
};
use anyhow::{anyhow, Context};
use clap::Parser;
use endless_sdk_builder::rust;
use endless_types::transaction::EntryABI;
use std::path::{Path, PathBuf};

pub const RELEASE_BUNDLE_EXTENSION: &str = "mrb";

/// Options to configure the generation of a release.
#[derive(Debug, Clone, Parser)]
#[clap(name = "Endless Releasing", author, version)]
pub struct ReleaseOptions {
    #[clap(flatten)]
    pub build_options: BuildOptions,
    /// The path to the Move packages for which to create a release.
    #[clap(long, value_parser, num_args(1..))]
    pub packages: Vec<PathBuf>,
    /// The path where to place generated Rust bindings for this module, in order for
    /// each package. If the value is empty (`""`) for a particular package, no bindings are
    /// generated.
    #[clap(long)]
    pub rust_bindings: Vec<String>,
    /// The path to the file where to place the release bundle.
    #[clap(long, default_value = "head.mrb", value_parser)]
    pub output: PathBuf,
}

impl ReleaseOptions {
    /// Creates a release bundle from the specified options and saves it to disk. As a side
    /// effect, also generates rust bindings.
    pub fn create_release(self) -> anyhow::Result<()> {
        let ReleaseOptions {
            build_options,
            packages,
            rust_bindings,
            output,
        } = self;
        let mut released_packages = vec![];
        let mut source_paths = vec![];
        for (package_path, rust_binding_path) in packages.into_iter().zip(rust_bindings.into_iter())
        {
            let built = BuiltPackage::build(package_path.clone(), build_options.clone())
                .with_context(|| {
                    format!(
                        "Failed to build package at path: {}",
                        package_path.display()
                    )
                })?;
            if !rust_binding_path.is_empty() {
                let abis = built
                    .extract_abis()
                    .ok_or_else(|| anyhow!("ABIs not available, can't generate sdk"))?;
                let binding_path = rust_binding_path.clone();
                Self::generate_rust_bindings(&abis, &PathBuf::from(rust_binding_path))
                    .with_context(|| {
                        format!(
                            "Failed to generate Rust bindings for {} at binding path {}",
                            package_path.display(),
                            binding_path
                        )
                    })?;
            }
            let released = ReleasePackage::new(built)?;
            let size = bcs::to_bytes(&released)?.len();
            println!(
                "Including package `{}` size {}k",
                released.name(),
                size / 1000,
            );
            released_packages.push(released);
            let relative_path = path_relative_to_crate(package_path.join("sources"));
            source_paths.push(relative_path.display().to_string());
        }
        let bundle = ReleaseBundle::new(released_packages, source_paths);
        let parent = output.parent().expect("Failed to get parent directory");
        std::fs::create_dir_all(parent).context("Failed to create dirs")?;
        std::fs::write(&output, bcs::to_bytes(&bundle)?).context("Failed to write output")?;
        Ok(())
    }

    fn generate_rust_bindings(abis: &[EntryABI], path: &Path) -> anyhow::Result<()> {
        {
            let mut file = std::fs::File::create(path)
                .with_context(|| format!("Failed to create {}", path.display()))?;
            rust::output(&mut file, abis, true)
                .with_context(|| format!("Failed to output rust bindings to {}", path.display()))?;
        }
        std::process::Command::new("rustfmt")
            .arg("--config")
            .arg("imports_granularity=crate")
            .arg(path)
            .output()
            .with_context(|| {
                format!(
                    "Failed to run rustfmt on {}, is rustfmt installed?",
                    path.display()
                )
            })?;
        Ok(())
    }
}
