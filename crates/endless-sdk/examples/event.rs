// Copyright © Endless
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use endless_logger::info;
use endless_rest_client::{Client, QueryRange};

#[tokio::main]
async fn main() -> Result<()> {
    endless_logger::Logger::new().init();

    let client = Client::new_testnet();
    let results = client
        .get_events_by_type("0x1::block::NewBlockEvent", QueryRange::by_version(1..=10))
        .await
        .context("Failed get_events_by_type_tag")?;
    info!(
        "Successfully retrieved {} events by type tag with JSON",
        results.inner().len()
    );

    let results = client
        .get_events_by_type_bcs("0x1::block::NewBlockEvent", QueryRange::by_version(1..=10))
        .await
        .context("Failed get_events_by_type_bcs")?;
    info!(
        "Successfully retrieved {} events by type tag with BCS",
        results.inner().len()
    );

    Ok(())
}
