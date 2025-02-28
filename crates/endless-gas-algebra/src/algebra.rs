// Copyright © Endless
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

pub use move_core_types::gas_algebra::*;

/***************************************************************************************************
 * Units & Quantities
 *
 **************************************************************************************************/
/// Unit of abstract value size -- a conceptual measurement of the memory space a Move value occupies.
pub enum AbstractValueUnit {}

pub type AbstractValueSize = GasQuantity<AbstractValueUnit>;

pub type InternalGasPerAbstractValueUnit = GasQuantity<UnitDiv<InternalGasUnit, AbstractValueUnit>>;

pub type AbstractValueSizePerArg = GasQuantity<UnitDiv<AbstractValueUnit, Arg>>;

/// Unit of (external) gas.
pub enum GasUnit {}

/// Unit of the Endless network's native coin.
pub enum EDS {}

/// Alternative unit of the Endless network's native coin. 1 Vein = 10^-8 Endless coins.
pub enum Vein {}

pub type Gas = GasQuantity<GasUnit>;

pub type GasScalingFactor = GasQuantity<UnitDiv<InternalGasUnit, GasUnit>>;

pub type Fee = GasQuantity<Vein>;

pub type FeePerGasUnit = GasQuantity<UnitDiv<Vein, GasUnit>>;

/// Unit of storage slot
pub enum Slot {}

pub type NumSlots = GasQuantity<Slot>;

pub type FeePerSlot = GasQuantity<UnitDiv<Vein, Slot>>;

pub type FeePerByte = GasQuantity<UnitDiv<Vein, Byte>>;

/***************************************************************************************************
 * Unit Conversion
 *
 **************************************************************************************************/
impl ToUnit<Vein> for EDS {
    const MULTIPLIER: u64 = 1_0000_0000;
}
