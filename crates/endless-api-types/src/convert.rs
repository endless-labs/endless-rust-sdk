// Copyright © Endless
// Copyright © Aptos Foundation

// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::Bytecode;
use anyhow::Result;
use endless_types::{transaction::ExecutionStatus, vm_status::AbortLocation};
use move_binary_format::file_format::FunctionHandleIndex;
use move_core_types::language_storage::ModuleId;
use std::rc::Rc;

pub fn new_vm_utf8_string(string: &str) -> move_core_types::value::MoveValue {
    use move_core_types::value::{MoveStruct, MoveValue};

    let byte_vector = MoveValue::Vector(
        string
            .as_bytes()
            .iter()
            .map(|byte| MoveValue::U8(*byte))
            .collect(),
    );
    let move_string = MoveStruct::Runtime(vec![byte_vector]);
    MoveValue::Struct(move_string)
}

fn abort_location_to_str(loc: &AbortLocation) -> String {
    match loc {
        AbortLocation::Module(mid) => {
            format!("{}::{}", mid.address().to_hex_literal(), mid.name())
        },
        _ => loc.to_string(),
    }
}

pub trait ExplainVMStatus {
    fn get_module_bytecode(&self, module_id: &ModuleId) -> Result<Rc<dyn Bytecode>>;

    fn explain_vm_status(&self, status: &ExecutionStatus) -> String {
        match status {
            ExecutionStatus::MoveAbort { location, code, info } => match &location {
                AbortLocation::Module(_) => {
                    info.as_ref().map(|i| {
                        format!("Move abort in {}: {}({:#x}): {}", abort_location_to_str(location), i.reason_name, code, i.description)
                    }).unwrap_or_else(|| {
                        format!("Move abort in {}: {:#x}", abort_location_to_str(location), code)
                    })
                }
                AbortLocation::Script => format!("Move abort: code {:#x}", code),
            },
            ExecutionStatus::Success => "Executed successfully".to_owned(),
            ExecutionStatus::OutOfGas => "Out of gas".to_owned(),
            ExecutionStatus::ExecutionFailure {
                location,
                function,
                code_offset,
            } => {
                let func_name = match location {
                    AbortLocation::Module(module_id) => self.explain_function_index(module_id, function)
                        .map(|name| format!("{}::{}", abort_location_to_str(location), name))
                        .unwrap_or_else(|_| format!("{}::<#{} function>", abort_location_to_str(location), function)),
                    AbortLocation::Script => "script".to_owned(),
                };
                format!(
                    "Execution failed in {} at code offset {}",
                    func_name, code_offset
                )
            }
            ExecutionStatus::MiscellaneousError(code) => {
                code.map_or(
                    "Move bytecode deserialization / verification failed, including entry function not found or invalid arguments".to_owned(),
                    |e| format!(
                        "Transaction Executed and Committed with Error {:#?}", e
                    ),
                )
            }
            ExecutionStatus::PaymentChecksumMismatch => "Payment Checksum Mismatch".to_owned(),
        }
    }

    fn explain_function_index(&self, module_id: &ModuleId, function: &u16) -> Result<String> {
        let code = self.get_module_bytecode(module_id)?;
        let func = code.function_handle_at(FunctionHandleIndex::new(*function));
        let id = code.identifier_at(func.name);
        Ok(id.to_string())
    }
}

// TODO: add caching?
