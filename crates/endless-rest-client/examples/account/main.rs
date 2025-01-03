// Copyright © Endless
// Copyright © Aptos Foundation

// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use clap::Parser;
use endless_logger::{debug, info};
use endless_rest_client::{Client, QueryRange};
use endless_types::{account_address::AccountAddress, chain_id::ChainId};
use reqwest::Url;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// This should include the port, e.g. http://127.0.0.1:8080
    #[clap(long)]
    api_url: Url,
}

// It isn't great to have all these tests together like this, but it's an okay
// start given we had nothing at all prior to this.
#[tokio::main]
async fn main() -> Result<()> {
    endless_logger::Logger::new().init();

    let args = Args::parse();
    debug!("Running with args: {:#?}", args);

    let client = Client::new(args.api_url);

    let address = AccountAddress::ONE;
    info!("Running all queries against account: {}", address);

    let results = client
        .get_account_resources(address)
        .await
        .context("Failed get_account_resources")?;
    info!(
        "Successfully retrieved {} account resources with JSON",
        results.inner().len()
    );

    let results = client
        .get_account_resources_bcs(address)
        .await
        .context("Failed get_account_resources_bcs")?;
    info!(
        "Successfully retrieved {} account resources with BCS",
        results.inner().len()
    );

    let results = client
        .get_account_modules(address)
        .await
        .context("Failed get_account_modules")?;
    info!(
        "Successfully retrieved {} account modules with JSON",
        results.inner().len()
    );

    let results = client
        .get_account_modules_bcs(address)
        .await
        .context("Failed get_account_modules_bcs")?;
    info!(
        "Successfully retrieved {} account modules with BCS",
        results.inner().len()
    );

    let resource = "0x1::chain_id::ChainId";

    client
        .get_account_resource(address, resource)
        .await
        .context("Failed get_account_resource")?;
    info!("Successfully retrieved resource {} with JSON", resource);

    client
        .get_account_resource_bcs::<ChainId>(address, resource)
        .await
        .context("Failed get_account_resource_bcs")?;
    info!("Successfully retrieved resource {} with BCS", resource);

    let results = client
        .get_events_by_type("0x1::block::NewBlockEvent", QueryRange::by_version(100..))
        .await
        .context("Failed get_events_by_type_tag")?;
    info!(
        "Successfully retrieved {} events by type tag with JSON",
        results.inner().len()
    );

    let results = client
        .get_events_by_type_bcs("0x1::block::NewBlockEvent", QueryRange::by_version(..=100))
        .await
        .context("Failed get_events_by_type_bcs")?;
    info!(
        "Successfully retrieved {} events by type tag with BCS",
        results.inner().len()
    );

    let results = client
        .get_transactions_by_version(vec![1, 2, 3])
        .await
        .context("Failed get_transactions_by_version")?;
    info!(
        "Successfully retrieved {} transactions by version with JSON",
        results.inner().len(),
    );

    let results = client
        .get_transactions_by_version_bcs(vec![1, 2, 3])
        .await
        .context("Failed get_transactions_by_version")?;
    info!(
        "Successfully retrieved {} transactions by version with BCS",
        results.inner().len(),
    );

    Ok(())
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
