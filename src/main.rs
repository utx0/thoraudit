//! bondy is a simple little script to find the bond amount
//! at the time the thornode went from standby to active in the current churn cycle
//!
//! usage: cargo run thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z

use std::env;
use std::process::exit;
use serde::Deserialize;
use serde_json::json;
use serde_derive::Deserialize;

const BASE_URL: &str = "https://thornode.thorchain.info/thorchain/node";

#[derive(Debug, Deserialize)]
struct NodeInfo {
    node_address: String,
    status: String,
    bond: String,
    bond_address: String,
    active_block_height: u32,
    status_since: u32,
    version: String,
    current_award: String,
    slash_points: u32,
    preflight_status: PreflightStatus,
}

#[derive(Debug, Deserialize)]
struct PreflightStatus {
    status: String,
    reason: String,
    code: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, bondy finder");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Error: No Node argument provided");
        exit(99);
    }

    let node = &args[1];

    let url = format!("{}/{}",BASE_URL,node);

    println!("url: {}", url);

    let responce= reqwest::get(url).await?.json::<NodeInfo>().await?;

    println!("{:#?}", responce);

    Ok(())
}
