// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    framework::{run_test_impl, CompiledState, MoveTestAdapter},
    tasks::{EmptyCommand, InitCommand, SyntaxChoice, TaskInput},
};
use anyhow::{anyhow, Result};
use clap::Parser;
use move_binary_format::{
    compatibility::Compatibility,
    errors::{PartialVMError, VMResult},
    file_format::CompiledScript,
    file_format_common,
    file_format_common::VERSION_MAX,
    CompiledModule,
};
use move_command_line_common::{
    address::ParsedAddress, env::read_bool_env_var, files::verify_and_create_named_address_mapping,
};
use move_compiler::{
    compiled_unit::AnnotatedCompiledUnit,
    shared::{known_attributes::KnownAttribute, Flags, PackagePaths},
    FullyCompiledProgram,
};
use move_core_types::{
    account_address::AccountAddress,
    identifier::{IdentStr, Identifier},
    language_storage::{ModuleId, StructTag, TypeTag},
    resolver::MoveResolver,
    value::MoveValue,
};
use move_resource_viewer::MoveValueAnnotator;
use move_stdlib::move_stdlib_named_addresses;
use move_symbol_pool::Symbol;
use move_vm_runtime::{
    config::VMConfig,
    move_vm::MoveVM,
    session::{SerializedReturnValues, Session},
};
use move_vm_test_utils::{gas_schedule::GasStatus, InMemoryStorage};
use once_cell::sync::Lazy;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

const STD_ADDR: AccountAddress = AccountAddress::ONE;

struct SimpleVMTestAdapter<'a> {
    compiled_state: CompiledState<'a>,
    storage: InMemoryStorage,
    default_syntax: SyntaxChoice,
    comparison_mode: bool,
    run_config: TestRunConfig,
}

pub fn view_resource_in_move_storage(
    storage: &impl MoveResolver<PartialVMError>,
    address: AccountAddress,
    module: &ModuleId,
    resource: &IdentStr,
    type_args: Vec<TypeTag>,
) -> Result<String> {
    let tag = StructTag {
        address: *module.address(),
        module: module.name().to_owned(),
        name: resource.to_owned(),
        type_params: type_args,
    };
    // TODO
    match storage.get_resource(&address, &tag).unwrap() {
        None => Ok("[No Resource Exists]".to_owned()),
        Some(data) => {
            let annotated = MoveValueAnnotator::new_with_max_bytecode_version(storage, VERSION_MAX)
                .view_resource(&tag, &data)?;
            Ok(format!("{}", annotated))
        },
    }
}

#[derive(Debug, Parser)]
pub struct AdapterPublishArgs {
    #[clap(long)]
    /// is skip the struct_and_pub_function_linking compatibility check
    pub skip_check_struct_and_pub_function_linking: bool,
    #[clap(long)]
    /// is skip the struct_layout compatibility check
    pub skip_check_struct_layout: bool,
    #[clap(long)]
    /// is skip the check friend link, if true, treat `friend` as `private`
    pub skip_check_friend_linking: bool,
    /// print more complete information for VMErrors on publish
    #[clap(long)]
    pub verbose: bool,
}

#[derive(Debug, Parser)]
pub struct AdapterExecuteArgs {
    #[clap(long)]
    pub check_runtime_types: bool,
    /// print more complete information for VMErrors on run
    #[clap(long)]
    pub verbose: bool,
}

fn move_test_debug() -> bool {
    static MOVE_TEST_DEBUG: Lazy<bool> = Lazy::new(|| read_bool_env_var("MOVE_TEST_DEBUG"));
    *MOVE_TEST_DEBUG
}

impl<'a> MoveTestAdapter<'a> for SimpleVMTestAdapter<'a> {
    type ExtraInitArgs = EmptyCommand;
    type ExtraPublishArgs = AdapterPublishArgs;
    type ExtraRunArgs = AdapterExecuteArgs;
    type ExtraValueArgs = ();
    type Subcommand = EmptyCommand;

    fn compiled_state(&mut self) -> &mut CompiledState<'a> {
        &mut self.compiled_state
    }

    fn default_syntax(&self) -> SyntaxChoice {
        self.default_syntax
    }

    fn known_attributes(&self) -> &BTreeSet<String> {
        KnownAttribute::get_all_attribute_names()
    }

    fn run_config(&self) -> TestRunConfig {
        self.run_config
    }

    fn init(
        default_syntax: SyntaxChoice,
        comparison_mode: bool,
        run_config: TestRunConfig,
        pre_compiled_deps: Option<&'a FullyCompiledProgram>,
        task_opt: Option<TaskInput<(InitCommand, EmptyCommand)>>,
    ) -> (Self, Option<String>) {
        let additional_mapping = match task_opt.map(|t| t.command) {
            Some((InitCommand { named_addresses }, _)) => {
                verify_and_create_named_address_mapping(named_addresses).unwrap()
            },
            None => BTreeMap::new(),
        };

        let mut named_address_mapping = move_stdlib_named_addresses();
        for (name, addr) in additional_mapping {
            if named_address_mapping.contains_key(&name) {
                panic!(
                    "Invalid init. The named address '{}' is reserved by the move-stdlib",
                    name
                )
            }
            named_address_mapping.insert(name, addr);
        }
        let mut adapter = Self {
            compiled_state: CompiledState::new(named_address_mapping, pre_compiled_deps, None),
            default_syntax,
            comparison_mode,
            run_config,
            storage: InMemoryStorage::new(),
        };

        adapter
            .perform_session_action(
                None,
                |session, gas_status| {
                    for module in &*MOVE_STDLIB_COMPILED {
                        let mut module_bytes = vec![];
                        module
                            .serialize_for_version(
                                Some(file_format_common::VERSION_MAX),
                                &mut module_bytes,
                            )
                            .unwrap();
                        let id = module.self_id();
                        let sender = *id.address();
                        session
                            .publish_module(module_bytes, sender, gas_status)
                            .unwrap();
                    }
                    Ok(())
                },
                VMConfig::production(),
            )
            .unwrap();
        let mut addr_to_name_mapping = BTreeMap::new();
        for (name, addr) in move_stdlib_named_addresses() {
            let prev = addr_to_name_mapping.insert(addr, Symbol::from(name));
            assert!(prev.is_none());
        }
        for module in MOVE_STDLIB_COMPILED
            .iter()
            .filter(|module| !adapter.compiled_state.is_precompiled_dep(&module.self_id()))
            .collect::<Vec<_>>()
        {
            adapter
                .compiled_state
                .add_and_generate_interface_file(module.clone());
        }
        (adapter, None)
    }

    fn publish_module(
        &mut self,
        module: CompiledModule,
        _named_addr_opt: Option<Identifier>,
        gas_budget: Option<u64>,
        extra_args: Self::ExtraPublishArgs,
    ) -> Result<(Option<String>, CompiledModule)> {
        let mut module_bytes = vec![];
        module.serialize_for_version(Some(file_format_common::VERSION_MAX), &mut module_bytes)?;

        let id = module.self_id();
        let sender = *id.address();
        let verbose = extra_args.verbose;
        match self.perform_session_action(
            gas_budget,
            |session, gas_status| {
                let compat = Compatibility::new(
                    !extra_args.skip_check_struct_and_pub_function_linking,
                    !extra_args.skip_check_struct_layout,
                    !extra_args.skip_check_friend_linking,
                );

                session.publish_module_bundle_with_compat_config(
                    vec![module_bytes],
                    sender,
                    gas_status,
                    compat,
                )
            },
            VMConfig::production(),
        ) {
            Ok(()) => Ok((None, module)),
            Err(vm_error) => Err(anyhow!(
                "Unable to publish module '{}'. Got VMError: {}",
                module.self_id(),
                vm_error.format_test_output(
                    move_test_debug() || verbose,
                    !move_test_debug() && self.comparison_mode
                )
            )),
        }
    }

    fn execute_script(
        &mut self,
        script: CompiledScript,
        type_args: Vec<TypeTag>,
        signers: Vec<ParsedAddress>,
        txn_args: Vec<MoveValue>,
        gas_budget: Option<u64>,
        extra_args: Self::ExtraRunArgs,
    ) -> Result<(Option<String>, SerializedReturnValues)> {
        let signers: Vec<_> = signers
            .into_iter()
            .map(|addr| self.compiled_state().resolve_address(&addr))
            .collect();

        let mut script_bytes = vec![];
        script.serialize_for_version(Some(file_format_common::VERSION_MAX), &mut script_bytes)?;

        let args = txn_args
            .iter()
            .map(|arg| arg.simple_serialize().unwrap())
            .collect::<Vec<_>>();
        // TODO rethink testing signer args
        let args = signers
            .iter()
            .map(|a| MoveValue::Signer(*a).simple_serialize().unwrap())
            .chain(args)
            .collect();
        let verbose = extra_args.verbose;
        let serialized_return_values = self
            .perform_session_action(
                gas_budget,
                |session, gas_status| {
                    session.execute_script(script_bytes, type_args, args, gas_status)
                },
                VMConfig::from(extra_args),
            )
            .map_err(|vm_error| {
                anyhow!(
                    "Script execution failed with VMError: {}",
                    vm_error.format_test_output(
                        move_test_debug() || verbose,
                        !move_test_debug() && self.comparison_mode
                    )
                )
            })?;
        Ok((None, serialized_return_values))
    }

    fn call_function(
        &mut self,
        module: &ModuleId,
        function: &IdentStr,
        type_args: Vec<TypeTag>,
        signers: Vec<ParsedAddress>,
        txn_args: Vec<MoveValue>,
        gas_budget: Option<u64>,
        extra_args: Self::ExtraRunArgs,
    ) -> Result<(Option<String>, SerializedReturnValues)> {
        let signers: Vec<_> = signers
            .into_iter()
            .map(|addr| self.compiled_state().resolve_address(&addr))
            .collect();

        let args = txn_args
            .iter()
            .map(|arg| arg.simple_serialize().unwrap())
            .collect::<Vec<_>>();
        // TODO rethink testing signer args
        let args = signers
            .iter()
            .map(|a| MoveValue::Signer(*a).simple_serialize().unwrap())
            .chain(args)
            .collect();
        let verbose = extra_args.verbose;
        let serialized_return_values = self
            .perform_session_action(
                gas_budget,
                |session, gas_status| {
                    session.execute_function_bypass_visibility(
                        module, function, type_args, args, gas_status,
                    )
                },
                VMConfig::from(extra_args),
            )
            .map_err(|vm_error| {
                anyhow!(
                    "Function execution failed with VMError: {}",
                    vm_error.format_test_output(
                        move_test_debug() || verbose,
                        !move_test_debug() && self.comparison_mode
                    )
                )
            })?;
        Ok((None, serialized_return_values))
    }

    fn view_data(
        &mut self,
        address: AccountAddress,
        module: &ModuleId,
        resource: &IdentStr,
        type_args: Vec<TypeTag>,
    ) -> Result<String> {
        view_resource_in_move_storage(&self.storage, address, module, resource, type_args)
    }

    fn handle_subcommand(&mut self, _: TaskInput<Self::Subcommand>) -> Result<Option<String>> {
        unreachable!()
    }
}

impl SimpleVMTestAdapter<'_> {
    fn perform_session_action<Ret>(
        &mut self,
        gas_budget: Option<u64>,
        f: impl FnOnce(&mut Session, &mut GasStatus) -> VMResult<Ret>,
        vm_config: VMConfig,
    ) -> VMResult<Ret> {
        // start session
        let vm = MoveVM::new_with_config(
            move_stdlib::natives::all_natives(
                STD_ADDR,
                // TODO: come up with a suitable gas schedule
                move_stdlib::natives::GasParameters::zeros(),
            ),
            vm_config,
        )
        .unwrap();
        let (mut session, mut gas_status) = {
            let gas_status = move_cli::sandbox::utils::get_gas_status(
                &move_vm_test_utils::gas_schedule::INITIAL_COST_SCHEDULE,
                gas_budget,
            )
            .unwrap();
            let session = vm.new_session(&self.storage);
            (session, gas_status)
        };

        // perform op
        let res = f(&mut session, &mut gas_status)?;

        // save changeset
        let changeset = session.finish()?;
        self.storage.apply(changeset).unwrap();
        Ok(res)
    }
}

static PRECOMPILED_MOVE_STDLIB: Lazy<FullyCompiledProgram> = Lazy::new(|| {
    let program_res = move_compiler::construct_pre_compiled_lib(
        vec![PackagePaths {
            name: None,
            paths: move_stdlib::move_stdlib_files(),
            named_address_map: move_stdlib::move_stdlib_named_addresses(),
        }],
        None,
        Flags::empty().set_skip_attribute_checks(true), // no point in checking.
        KnownAttribute::get_all_attribute_names(),
    )
    .unwrap();
    match program_res {
        Ok(stdlib) => stdlib,
        Err((files, errors)) => {
            eprintln!("!!!Standard library failed to compile!!!");
            move_compiler::diagnostics::report_diagnostics(&files, errors)
        },
    }
});

static MOVE_STDLIB_COMPILED: Lazy<Vec<CompiledModule>> = Lazy::new(|| {
    let (files, units_res) = move_compiler::Compiler::from_files(
        move_stdlib::move_stdlib_files(),
        vec![],
        move_stdlib::move_stdlib_named_addresses(),
        Flags::empty().set_skip_attribute_checks(true), // no point in checking here.
        KnownAttribute::get_all_attribute_names(),
    )
    .build()
    .unwrap();
    match units_res {
        Err(diags) => {
            eprintln!("!!!Standard library failed to compile!!!");
            move_compiler::diagnostics::report_diagnostics(&files, diags)
        },
        Ok((_, warnings)) if !warnings.is_empty() => {
            eprintln!("!!!Standard library failed to compile!!!");
            move_compiler::diagnostics::report_diagnostics(&files, warnings)
        },
        Ok((units, _warnings)) => units
            .into_iter()
            .filter_map(|m| match m {
                AnnotatedCompiledUnit::Module(annot_module) => {
                    Some(annot_module.named_module.module)
                },
                AnnotatedCompiledUnit::Script(_) => None,
            })
            .collect(),
    }
});

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum TestRunConfig {
    CompilerV1,
    CompilerV2,
    ComparisonV1V2,
}

pub fn run_test(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    run_test_with_config(TestRunConfig::CompilerV1, path)
}

pub fn run_test_with_config(
    config: TestRunConfig,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    run_test_impl::<SimpleVMTestAdapter>(config, path, Some(&*PRECOMPILED_MOVE_STDLIB))
}

impl From<AdapterExecuteArgs> for VMConfig {
    fn from(arg: AdapterExecuteArgs) -> VMConfig {
        VMConfig {
            paranoid_type_checks: arg.check_runtime_types,
            ..Self::production()
        }
    }
}
