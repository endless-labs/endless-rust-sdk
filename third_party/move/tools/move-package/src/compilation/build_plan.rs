// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use super::package_layout::CompiledPackageLayout;
use crate::{
    compilation::compiled_package::{
        build_and_report_no_exit_v2_driver, build_and_report_v2_driver, CompiledPackage,
    },
    resolution::resolution_graph::ResolvedGraph,
    source_package::parsed_manifest::PackageName,
    CompilerConfig,
};
use anyhow::Result;
use move_compiler::{
    compiled_unit::AnnotatedCompiledUnit,
    diagnostics::{report_diagnostics_to_color_buffer, report_warnings, FilesSourceText},
    Compiler,
};
use move_model::model;
use petgraph::algo::toposort;
use std::{collections::BTreeSet, io::Write, path::Path};

#[derive(Debug, Clone)]
pub struct BuildPlan {
    root: PackageName,
    sorted_deps: Vec<PackageName>,
    resolution_graph: ResolvedGraph,
}

pub type CompilerDriverResult = anyhow::Result<(FilesSourceText, Vec<AnnotatedCompiledUnit>)>;

impl BuildPlan {
    pub fn create(resolution_graph: ResolvedGraph) -> Result<Self> {
        let mut sorted_deps = match toposort(&resolution_graph.graph, None) {
            Ok(nodes) => nodes,
            Err(err) => {
                // Is a DAG after resolution otherwise an error should be raised from that.
                anyhow::bail!("IPE: Cyclic dependency found after resolution {:?}", err)
            },
        };

        sorted_deps.reverse();

        Ok(Self {
            root: resolution_graph.root_package.package.name,
            sorted_deps,
            resolution_graph,
        })
    }

    /// Compilation results in the process exit upon warning/failure
    pub fn compile<W: Write>(
        &self,
        config: &CompilerConfig,
        writer: &mut W,
    ) -> Result<CompiledPackage> {
        self.compile_with_driver(
            writer,
            config,
            |compiler| compiler.build_and_report(),
            build_and_report_v2_driver,
        )
        .map(|(package, _)| package)
    }

    /// Compilation process does not exit even if warnings/failures are encountered
    pub fn compile_no_exit<W: Write>(
        &self,
        config: &CompilerConfig,
        writer: &mut W,
    ) -> Result<(CompiledPackage, Option<model::GlobalEnv>)> {
        self.compile_with_driver(
            writer,
            config,
            |compiler| {
                let (files, units_res) = compiler.build()?;
                match units_res {
                    Ok((units, warning_diags)) => {
                        report_warnings(&files, warning_diags);
                        Ok((files, units))
                    },
                    Err(error_diags) => {
                        assert!(!error_diags.is_empty());
                        let diags_buf = report_diagnostics_to_color_buffer(&files, error_diags);
                        if let Err(err) = std::io::stdout().write_all(&diags_buf) {
                            anyhow::bail!("Cannot output compiler diagnostics: {}", err);
                        }
                        anyhow::bail!("Compilation error");
                    },
                }
            },
            build_and_report_no_exit_v2_driver,
        )
    }

    pub fn compile_with_driver<W: Write>(
        &self,
        writer: &mut W,
        config: &CompilerConfig,
        compiler_driver_v1: impl FnMut(Compiler) -> CompilerDriverResult,
        compiler_driver_v2: impl FnMut(move_compiler_v2::Options) -> CompilerDriverResult,
    ) -> Result<(CompiledPackage, Option<model::GlobalEnv>)> {
        let root_package = &self.resolution_graph.package_table[&self.root];
        let project_root = match &self.resolution_graph.build_options.install_dir {
            Some(under_path) => under_path.clone(),
            None => self.resolution_graph.root_package_path.clone(),
        };
        let immediate_dependencies_names =
            root_package.immediate_dependencies(&self.resolution_graph);
        let transitive_dependencies = root_package
            .transitive_dependencies(&self.resolution_graph)
            .into_iter()
            .map(|package_name| {
                let dep_package = self
                    .resolution_graph
                    .package_table
                    .get(&package_name)
                    .unwrap();
                let mut dep_source_paths = dep_package
                    .get_sources(&self.resolution_graph.build_options)
                    .unwrap();
                let mut source_available = true;
                // If source is empty, search bytecode(mv) files
                if dep_source_paths.is_empty() {
                    dep_source_paths = dep_package.get_bytecodes().unwrap();
                    source_available = false;
                }
                (
                    package_name,
                    immediate_dependencies_names.contains(&package_name),
                    dep_source_paths,
                    &dep_package.resolution_table,
                    source_available,
                )
            })
            .collect();

        let (compiled, model) = CompiledPackage::build_all(
            writer,
            &project_root,
            root_package.clone(),
            transitive_dependencies,
            config,
            &self.resolution_graph,
            compiler_driver_v1,
            compiler_driver_v2,
        )?;

        Self::clean(
            &project_root.join(CompiledPackageLayout::Root.path()),
            self.sorted_deps.iter().copied().collect(),
        )?;
        Ok((compiled, model))
    }

    // Clean out old packages that are no longer used, or no longer used under the current
    // compilation flags
    fn clean(build_root: &Path, keep_paths: BTreeSet<PackageName>) -> Result<()> {
        for dir in std::fs::read_dir(build_root)? {
            let path = dir?.path();
            if !keep_paths.iter().any(|name| path.ends_with(name.as_str())) {
                std::fs::remove_dir_all(&path)?;
            }
        }
        Ok(())
    }
}
