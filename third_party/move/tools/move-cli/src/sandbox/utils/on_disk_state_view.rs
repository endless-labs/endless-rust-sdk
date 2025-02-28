// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{BCS_EXTENSION, DEFAULT_BUILD_DIR, DEFAULT_STORAGE_DIR};
use anyhow::{anyhow, bail, Result};
use bytes::Bytes;
use move_binary_format::{
    access::ModuleAccess,
    binary_views::BinaryIndexedView,
    errors::PartialVMError,
    file_format::{CompiledModule, CompiledScript, FunctionDefinitionIndex},
};
use move_bytecode_utils::module_cache::GetModule;
use move_command_line_common::files::MOVE_COMPILED_EXTENSION;
use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    language_storage::{ModuleId, StructTag, TypeTag},
    metadata::Metadata,
    parser,
    resolver::{resource_size, ModuleResolver, ResourceResolver},
    value::MoveTypeLayout,
    vm_status::StatusCode,
};
use move_disassembler::disassembler::Disassembler;
use move_ir_types::location::Spanned;
use move_resource_viewer::{AnnotatedMoveStruct, MoveValueAnnotator};
use std::{
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

/// subdirectory of `DEFAULT_STORAGE_DIR/<addr>` where resources are stored
pub const RESOURCES_DIR: &str = "resources";
/// subdirectory of `DEFAULT_STORAGE_DIR/<addr>` where modules are stored
pub const MODULES_DIR: &str = "modules";

/// file under `DEFAULT_BUILD_DIR` where a registry of generated struct layouts are stored
pub const STRUCT_LAYOUTS_FILE: &str = "struct_layouts.yaml";

#[derive(Debug)]
pub struct OnDiskStateView {
    build_dir: PathBuf,
    storage_dir: PathBuf,
}

impl OnDiskStateView {
    /// Create an `OnDiskStateView` that reads/writes resource data and modules in `storage_dir`.
    pub fn create<P: Into<PathBuf>>(build_dir: P, storage_dir: P) -> Result<Self> {
        let build_dir = build_dir.into();
        if !build_dir.exists() {
            fs::create_dir_all(&build_dir)?;
        }

        let storage_dir = storage_dir.into();
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)?;
        }

        Ok(Self {
            build_dir,
            // it is important to canonicalize the path here because `is_data_path()` relies on the
            // fact that storage_dir is canonicalized.
            storage_dir: storage_dir.canonicalize()?,
        })
    }

    pub fn build_dir(&self) -> &PathBuf {
        &self.build_dir
    }

    pub fn struct_layouts_file(&self) -> PathBuf {
        self.build_dir.join(STRUCT_LAYOUTS_FILE)
    }

    fn is_data_path(&self, p: &Path, parent_dir: &str) -> bool {
        if !p.exists() {
            return false;
        }
        let p = p.canonicalize().unwrap();
        p.starts_with(&self.storage_dir)
            && match p.parent() {
                Some(parent) => parent.ends_with(parent_dir),
                None => false,
            }
    }

    pub fn is_resource_path(&self, p: &Path) -> bool {
        self.is_data_path(p, RESOURCES_DIR)
    }

    pub fn is_module_path(&self, p: &Path) -> bool {
        self.is_data_path(p, MODULES_DIR)
    }

    fn get_addr_path(&self, addr: &AccountAddress) -> PathBuf {
        let mut path = self.storage_dir.clone();
        path.push(format!("0x{}", addr.to_hex()));
        path
    }

    fn get_resource_path(&self, addr: AccountAddress, tag: StructTag) -> PathBuf {
        let mut path = self.get_addr_path(&addr);
        path.push(RESOURCES_DIR);
        path.push(StructID(tag).to_string());
        path.with_extension(BCS_EXTENSION)
    }

    fn get_module_path(&self, module_id: &ModuleId) -> PathBuf {
        let mut path = self.get_addr_path(module_id.address());
        path.push(MODULES_DIR);
        path.push(module_id.name().to_string());
        path.with_extension(MOVE_COMPILED_EXTENSION)
    }

    /// Extract a module ID from a path
    pub fn get_module_id(&self, p: &Path) -> Option<ModuleId> {
        if !self.is_module_path(p) {
            return None;
        }
        let name = Identifier::new(p.file_stem().unwrap().to_str().unwrap()).unwrap();
        match p.parent().and_then(|parent| parent.parent()) {
            Some(parent) => {
                let addr =
                    AccountAddress::from_hex_literal(parent.file_stem().unwrap().to_str().unwrap())
                        .unwrap();
                Some(ModuleId::new(addr, name))
            },
            None => None,
        }
    }

    /// Read the resource bytes stored on-disk at `addr`/`tag`
    pub fn get_resource_bytes(
        &self,
        addr: AccountAddress,
        tag: StructTag,
    ) -> Result<Option<Bytes>> {
        Self::get_bytes(&self.get_resource_path(addr, tag))
    }

    /// Read the resource bytes stored on-disk at `addr`/`tag`
    fn get_module_bytes(&self, module_id: &ModuleId) -> Result<Option<Bytes>> {
        Self::get_bytes(&self.get_module_path(module_id))
    }

    /// Check if a module at `addr`/`module_id` exists
    pub fn has_module(&self, module_id: &ModuleId) -> bool {
        self.get_module_path(module_id).exists()
    }

    /// Return the name of the function at `idx` in `module_id`
    pub fn resolve_function(&self, module_id: &ModuleId, idx: u16) -> Result<Option<Identifier>> {
        if let Some(m) = self.get_module_by_id(module_id)? {
            Ok(Some(
                m.identifier_at(
                    m.function_handle_at(m.function_def_at(FunctionDefinitionIndex(idx)).function)
                        .name,
                )
                .to_owned(),
            ))
        } else {
            Ok(None)
        }
    }

    fn get_bytes(path: &Path) -> Result<Option<Bytes>> {
        Ok(if path.exists() {
            Some(fs::read(path)?.into())
        } else {
            None
        })
    }

    /// Returns a deserialized representation of the resource value stored at `resource_path`.
    /// Returns Err if the path does not hold a resource value or the resource cannot be deserialized
    pub fn view_resource(&self, resource_path: &Path) -> Result<Option<AnnotatedMoveStruct>> {
        if resource_path.is_dir() {
            bail!(
                "Bad resource path {:?}. Needed file, found directory",
                resource_path
            )
        }
        match resource_path.file_stem() {
            None => bail!(
                "Bad resource path {:?}; last component must be a file",
                resource_path
            ),
            Some(name) => Ok({
                let id = match parser::parse_type_tag(&name.to_string_lossy())? {
                    TypeTag::Struct(s) => s,
                    t => bail!("Expected to parse struct tag, but got {}", t),
                };
                match Self::get_bytes(resource_path)? {
                    Some(resource_data) => {
                        Some(MoveValueAnnotator::new(self).view_resource(&id, &resource_data)?)
                    },
                    None => None,
                }
            }),
        }
    }

    fn view_bytecode(path: &Path, is_module: bool) -> Result<Option<String>> {
        if path.is_dir() {
            bail!("Bad bytecode path {:?}. Needed file, found directory", path)
        }

        Ok(match Self::get_bytes(path)? {
            Some(bytes) => {
                let module: CompiledModule;
                let script: CompiledScript;
                let view = if is_module {
                    module = CompiledModule::deserialize(&bytes)
                        .map_err(|e| anyhow!("Failure deserializing module: {:?}", e))?;
                    BinaryIndexedView::Module(&module)
                } else {
                    script = CompiledScript::deserialize(&bytes)
                        .map_err(|e| anyhow!("Failure deserializing script: {:?}", e))?;
                    BinaryIndexedView::Script(&script)
                };
                // TODO: find or create source map and pass it to disassembler
                let d: Disassembler =
                    Disassembler::from_view(view, Spanned::unsafe_no_loc(()).loc)?;
                Some(d.disassemble()?)
            },
            None => None,
        })
    }

    pub fn view_module(module_path: &Path) -> Result<Option<String>> {
        Self::view_bytecode(module_path, true)
    }

    pub fn view_script(script_path: &Path) -> Result<Option<String>> {
        Self::view_bytecode(script_path, false)
    }

    /// Delete resource stored on disk at the path `addr`/`tag`
    pub fn delete_resource(&self, addr: AccountAddress, tag: StructTag) -> Result<()> {
        let path = self.get_resource_path(addr, tag);
        fs::remove_file(path)?;

        // delete addr directory if this address is now empty
        let addr_path = self.get_addr_path(&addr);
        if addr_path.read_dir()?.next().is_none() {
            fs::remove_dir(addr_path)?
        }
        Ok(())
    }

    pub fn save_resource(
        &self,
        addr: AccountAddress,
        tag: StructTag,
        bcs_bytes: &[u8],
    ) -> Result<()> {
        let path = self.get_resource_path(addr, tag);
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap())?;
        }
        Ok(fs::write(path, bcs_bytes)?)
    }

    /// Save `module` on disk under the path `module.address()`/`module.name()`
    pub fn save_module(&self, module_id: &ModuleId, module_bytes: &[u8]) -> Result<()> {
        let path = self.get_module_path(module_id);
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap())?
        }
        Ok(fs::write(path, module_bytes)?)
    }

    /// Save the YAML encoding `layout` on disk under `build_dir/layouts/id`.
    pub fn save_struct_layouts(&self, layouts: &str) -> Result<()> {
        let layouts_file = self.struct_layouts_file();
        if !layouts_file.exists() {
            fs::create_dir_all(layouts_file.parent().unwrap())?
        }
        Ok(fs::write(layouts_file, layouts)?)
    }

    /// Save all the modules in the local cache, re-generate mv_interfaces if required.
    pub fn save_modules<'a>(
        &self,
        modules: impl IntoIterator<Item = &'a (ModuleId, Bytes)>,
    ) -> Result<()> {
        for (module_id, module_bytes) in modules {
            self.save_module(module_id, module_bytes)?;
        }
        Ok(())
    }

    pub fn delete_module(&self, id: &ModuleId) -> Result<()> {
        let path = self.get_module_path(id);
        fs::remove_file(path)?;

        // delete addr directory if this address is now empty
        let addr_path = self.get_addr_path(id.address());
        if addr_path.read_dir()?.next().is_none() {
            fs::remove_dir(addr_path)?
        }
        Ok(())
    }

    fn iter_paths<F>(&self, f: F) -> impl Iterator<Item = PathBuf>
    where
        F: FnOnce(&Path) -> bool + Copy,
    {
        walkdir::WalkDir::new(&self.storage_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .filter(move |path| f(path))
    }

    pub fn resource_paths(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.iter_paths(move |p| self.is_resource_path(p))
    }

    pub fn module_paths(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.iter_paths(move |p| self.is_module_path(p))
    }

    /// Build all modules in the self.storage_dir.
    /// Returns an Err if a module does not deserialize.
    pub fn get_all_modules(&self) -> Result<Vec<CompiledModule>> {
        self.module_paths()
            .map(|path| {
                CompiledModule::deserialize(&Self::get_bytes(&path)?.unwrap())
                    .map_err(|e| anyhow!("Failed to deserialized module: {:?}", e))
            })
            .collect::<Result<Vec<CompiledModule>>>()
    }
}

impl ModuleResolver for OnDiskStateView {
    type Error = PartialVMError;

    fn get_module_metadata(&self, _module_id: &ModuleId) -> Vec<Metadata> {
        vec![]
    }

    fn get_module(&self, module_id: &ModuleId) -> Result<Option<Bytes>, Self::Error> {
        self.get_module_bytes(module_id).map_err(|e| {
            PartialVMError::new(StatusCode::STORAGE_ERROR)
                .with_message(format!("Storage error: {:?}", e))
        })
    }
}

impl ResourceResolver for OnDiskStateView {
    type Error = PartialVMError;

    fn get_resource_bytes_with_metadata_and_layout(
        &self,
        address: &AccountAddress,
        struct_tag: &StructTag,
        _metadata: &[Metadata],
        _maybe_layout: Option<&MoveTypeLayout>,
    ) -> Result<(Option<Bytes>, usize), Self::Error> {
        let buf = self
            .get_resource_bytes(*address, struct_tag.clone())
            .map_err(|e| {
                PartialVMError::new(StatusCode::STORAGE_ERROR)
                    .with_message(format!("Storage error: {:?}", e))
            })?;
        let buf_size = resource_size(&buf);
        Ok((buf, buf_size))
    }
}

impl GetModule for &OnDiskStateView {
    type Error = anyhow::Error;
    type Item = CompiledModule;

    fn get_module_by_id(&self, id: &ModuleId) -> Result<Option<CompiledModule>, Self::Error> {
        if let Some(bytes) = self.get_module_bytes(id)? {
            let module = CompiledModule::deserialize(&bytes)
                .map_err(|e| anyhow!("Failure deserializing module {:?}: {:?}", id, e))?;
            Ok(Some(module))
        } else {
            Ok(None)
        }
    }
}

impl Default for OnDiskStateView {
    fn default() -> Self {
        OnDiskStateView::create(Path::new(DEFAULT_BUILD_DIR), Path::new(DEFAULT_STORAGE_DIR))
            .expect("Failure creating OnDiskStateView")
    }
}

// wrappers of TypeTag, StructTag, Vec<TypeTag> to allow us to implement the FromStr/ToString traits
#[derive(Debug)]
struct TypeID(TypeTag);
#[derive(Debug)]
struct StructID(StructTag);
#[derive(Debug)]
struct Generics(Vec<TypeTag>);

impl std::fmt::Display for TypeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            TypeTag::Struct(s) => write!(f, "{}", StructID(*s.clone())),
            TypeTag::Vector(t) => write!(f, "vector<{}>", TypeID(*t.clone())),
            t => write!(f, "{}", t),
        }
    }
}

impl std::fmt::Display for StructID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = &self.0;
        // TODO: TypeTag parser insists on leading 0x for StructTag's, so we insert one here.
        // Would be nice to expose a StructTag parser and get rid of the 0x here
        write!(
            f,
            "0x{}::{}::{}{}",
            tag.address.to_hex(),
            tag.module,
            tag.name,
            Generics(tag.type_params.clone())
        )
    }
}

impl std::fmt::Display for Generics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0.is_empty() {
            let generics: Vec<String> = self
                .0
                .iter()
                .map(|t| TypeID(t.clone()).to_string())
                .collect();
            write!(f, "<{}>", generics.join(","))
        } else {
            Ok(())
        }
    }
}
