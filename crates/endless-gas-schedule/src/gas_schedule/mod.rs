// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use crate::traits::{FromOnChainGasSchedule, InitialGasSchedule, ToOnChainGasSchedule};
use std::collections::BTreeMap;

mod endless_framework;
mod instr;
mod macros;
mod misc;
mod move_stdlib;
mod table;
mod transaction;

pub use endless_framework::EndlessFrameworkGasParameters;
pub use instr::InstructionGasParameters;
pub use misc::{AbstractValueSizeGasParameters, MiscGasParameters};
pub use move_stdlib::MoveStdlibGasParameters;
pub use table::TableGasParameters;
pub use transaction::TransactionGasParameters;

pub mod gas_params {
    use super::*;
    pub use instr::gas_params as instr;
    pub use misc::gas_params as misc;
    pub use transaction::gas_params as txn;

    pub mod natives {
        use super::*;
        pub use endless_framework::gas_params as endless_framework;
        pub use move_stdlib::gas_params as move_stdlib;
        pub use table::gas_params as table;
    }
}

/// Gas parameters for everything that is needed to run the Endless blockchain, including
/// instructions, transactions and native functions from various packages.
#[derive(Debug, Clone)]
pub struct EndlessGasParameters {
    pub vm: VMGasParameters,
    pub natives: NativeGasParameters,
}

impl FromOnChainGasSchedule for EndlessGasParameters {
    fn from_on_chain_gas_schedule(
        gas_schedule: &BTreeMap<String, u64>,
        feature_version: u64,
    ) -> Result<Self, String> {
        Ok(Self {
            vm: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule, feature_version)?,
            natives: FromOnChainGasSchedule::from_on_chain_gas_schedule(
                gas_schedule,
                feature_version,
            )?,
        })
    }
}

impl ToOnChainGasSchedule for EndlessGasParameters {
    fn to_on_chain_gas_schedule(&self, feature_version: u64) -> Vec<(String, u64)> {
        let mut entries = self.vm.to_on_chain_gas_schedule(feature_version);
        entries.extend(self.natives.to_on_chain_gas_schedule(feature_version));
        entries
    }
}

impl EndlessGasParameters {
    pub fn zeros() -> Self {
        Self {
            vm: VMGasParameters::zeros(),
            natives: NativeGasParameters::zeros(),
        }
    }
}

impl InitialGasSchedule for EndlessGasParameters {
    fn initial() -> Self {
        Self {
            vm: InitialGasSchedule::initial(),
            natives: InitialGasSchedule::initial(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VMGasParameters {
    pub misc: MiscGasParameters,
    pub instr: InstructionGasParameters,
    pub txn: TransactionGasParameters,
}

impl FromOnChainGasSchedule for VMGasParameters {
    fn from_on_chain_gas_schedule(
        gas_schedule: &BTreeMap<String, u64>,
        feature_version: u64,
    ) -> Result<Self, String> {
        Ok(Self {
            misc: FromOnChainGasSchedule::from_on_chain_gas_schedule(
                gas_schedule,
                feature_version,
            )?,
            instr: FromOnChainGasSchedule::from_on_chain_gas_schedule(
                gas_schedule,
                feature_version,
            )?,
            txn: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule, feature_version)?,
        })
    }
}

impl ToOnChainGasSchedule for VMGasParameters {
    fn to_on_chain_gas_schedule(&self, feature_version: u64) -> Vec<(String, u64)> {
        let mut entries = self.instr.to_on_chain_gas_schedule(feature_version);
        entries.extend(self.txn.to_on_chain_gas_schedule(feature_version));
        entries.extend(self.misc.to_on_chain_gas_schedule(feature_version));
        entries
    }
}

impl VMGasParameters {
    pub fn zeros() -> Self {
        Self {
            misc: MiscGasParameters::zeros(),
            instr: InstructionGasParameters::zeros(),
            txn: TransactionGasParameters::zeros(),
        }
    }
}

impl InitialGasSchedule for VMGasParameters {
    fn initial() -> Self {
        Self {
            misc: InitialGasSchedule::initial(),
            instr: InitialGasSchedule::initial(),
            txn: InitialGasSchedule::initial(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NativeGasParameters {
    pub move_stdlib: MoveStdlibGasParameters,
    pub table: TableGasParameters,
    pub endless_framework: EndlessFrameworkGasParameters,
}

impl FromOnChainGasSchedule for NativeGasParameters {
    fn from_on_chain_gas_schedule(
        gas_schedule: &BTreeMap<String, u64>,
        feature_version: u64,
    ) -> Result<Self, String> {
        Ok(Self {
            move_stdlib: FromOnChainGasSchedule::from_on_chain_gas_schedule(
                gas_schedule,
                feature_version,
            )?,
            table: FromOnChainGasSchedule::from_on_chain_gas_schedule(
                gas_schedule,
                feature_version,
            )?,
            endless_framework: FromOnChainGasSchedule::from_on_chain_gas_schedule(
                gas_schedule,
                feature_version,
            )?,
        })
    }
}

impl ToOnChainGasSchedule for NativeGasParameters {
    fn to_on_chain_gas_schedule(&self, feature_version: u64) -> Vec<(String, u64)> {
        let mut entries = self.move_stdlib.to_on_chain_gas_schedule(feature_version);
        entries.extend(self.table.to_on_chain_gas_schedule(feature_version));
        entries.extend(
            self.endless_framework
                .to_on_chain_gas_schedule(feature_version),
        );
        entries
    }
}

impl NativeGasParameters {
    pub fn zeros() -> Self {
        Self {
            move_stdlib: MoveStdlibGasParameters::zeros(),
            table: TableGasParameters::zeros(),
            endless_framework: EndlessFrameworkGasParameters::zeros(),
        }
    }
}

impl InitialGasSchedule for NativeGasParameters {
    fn initial() -> Self {
        Self {
            move_stdlib: InitialGasSchedule::initial(),
            table: InitialGasSchedule::initial(),
            endless_framework: InitialGasSchedule::initial(),
        }
    }
}
