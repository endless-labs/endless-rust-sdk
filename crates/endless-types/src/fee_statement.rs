// Copyright © Endless
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

/// Breakdown of fee charge and refund for a transaction.
/// The structure is:
///
/// - Net charge or refund (not in the statement)
///    - total charge: total_charge_gas_units, matches `gas_used` in the on-chain `TransactionInfo`.
///      This is the sum of the sub-items below. Notice that there's potential precision loss when
///      the conversion between internal and external gas units and between native token and gas
///      units, so it's possible that the numbers don't add up exactly. -- This number is the final
///      charge, while the break down is merely informational.
///        - gas charge for execution (CPU time): `execution_gas_units`
///        - gas charge for IO (storage random access): `io_gas_units`
///        - storage fee charge (storage space): `storage_fee_veins`, to be included in
///          `total_charge_gas_unit`, this number is converted to gas units according to the user
///          specified `gas_unit_price` on the transaction.
///    - storage deletion refund: `storage_fee_refund_veins`, this is not included in `gas_used` or
///      `total_charge_gas_units`, the net charge / refund is calculated by
///      `total_charge_gas_units` * `gas_unit_price` - `storage_fee_refund_veins`.
///
/// This is meant to emitted as a module event.
///
/// (keep this doc in sync with the `struct FeeStatement` in Move.)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeeStatement {
    /// Total gas charge.
    total_charge_gas_units: u64,
    /// Execution gas charge.
    execution_gas_units: u64,
    /// IO gas charge.
    io_gas_units: u64,
    /// Storage fee charge.
    storage_fee_veins: u64,
    /// Storage fee refund.
    storage_fee_refund_veins: u64,
}

impl FeeStatement {
    pub fn zero() -> Self {
        Self {
            total_charge_gas_units: 0,
            execution_gas_units: 0,
            io_gas_units: 0,
            storage_fee_veins: 0,
            storage_fee_refund_veins: 0,
        }
    }

    pub fn new(
        total_charge_gas_units: u64,
        execution_gas_units: u64,
        io_gas_units: u64,
        storage_fee_veins: u64,
        storage_fee_refund_veins: u64,
    ) -> Self {
        Self {
            total_charge_gas_units,
            execution_gas_units,
            io_gas_units,
            storage_fee_veins,
            storage_fee_refund_veins,
        }
    }

    pub fn clear_refunds(&mut self) {
        self.storage_fee_refund_veins = 0;
    }

    pub fn gas_used(&self) -> u64 {
        self.total_charge_gas_units
    }

    pub fn execution_gas_used(&self) -> u64 {
        self.execution_gas_units
    }

    pub fn io_gas_used(&self) -> u64 {
        self.io_gas_units
    }

    pub fn storage_fee_used(&self) -> u64 {
        self.storage_fee_veins
    }

    pub fn storage_fee_refund(&self) -> u64 {
        self.storage_fee_refund_veins
    }

    pub fn add_fee_statement(&mut self, other: &FeeStatement) {
        self.total_charge_gas_units += other.total_charge_gas_units;
        self.execution_gas_units += other.execution_gas_units;
        self.io_gas_units += other.io_gas_units;
        self.storage_fee_veins += other.storage_fee_veins;
        self.storage_fee_refund_veins += other.storage_fee_refund_veins;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeeStatementEvent {
    /// Total gas charge.
    total_charge_gas_units: u64,
    /// Execution gas charge.
    execution_gas_units: u64,
    /// IO gas charge.
    io_gas_units: u64,
    /// Storage fee charge.
    storage_fee_veins: u64,
    /// Storage fee refund.
    storage_fee_refund_veins: u64,
    /// Gas payer
    gas_payer: AccountAddress,
}

impl FeeStatementEvent {
    pub fn new(fee_statement: FeeStatement, gas_payer: AccountAddress) -> Self {
        Self {
            total_charge_gas_units: fee_statement.total_charge_gas_units,
            execution_gas_units: fee_statement.execution_gas_units,
            io_gas_units: fee_statement.io_gas_units,
            storage_fee_veins: fee_statement.storage_fee_veins,
            storage_fee_refund_veins: fee_statement.storage_fee_refund_veins,
            gas_payer,
        }
    }

    pub fn clear_refunds(&mut self) {
        self.storage_fee_refund_veins = 0;
    }

    pub fn gas_used(&self) -> u64 {
        self.total_charge_gas_units
    }

    pub fn execution_gas_used(&self) -> u64 {
        self.execution_gas_units
    }

    pub fn io_gas_used(&self) -> u64 {
        self.io_gas_units
    }

    pub fn storage_fee_used(&self) -> u64 {
        self.storage_fee_veins
    }

    pub fn storage_fee_refund(&self) -> u64 {
        self.storage_fee_refund_veins
    }

    pub fn gas_payer(&self) -> AccountAddress {
        self.gas_payer
    }
}
