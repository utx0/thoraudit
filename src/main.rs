//! thoraudit is a simple little script to find the bond amount
//! at the time the thornode went from standby to active in the current churn cycle
//!
//! usage: cargo run thor1h0xd53y8yvfsu5t8p6t4ky52h9dzfz3mvsem3z

use std::collections::HashMap;
use std::env;
use std::process::exit;
use serde_derive::Deserialize;

const BASE_URL: &str = "https://thornode.thorchain.info/thorchain/node";

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
struct PreflightStatus {
    status: String,
    reason: String,
    code: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Error: No Node argument provided");
        exit(99);
    }

    let node = &args[1];

    let url = format!("{}/{}",BASE_URL,node);

    println!("url: {}", url);

    let head_responce= reqwest::get(url).await?.json::<NodeInfo>().await?;
    let mut node_infos = HashMap::new();

    node_infos.insert(head_responce.status_since, head_responce.clone());

    println!("{:#?}", head_responce);

    let mut status_since = head_responce.status_since;

    while status_since != 0 {
        let node_info = getblock(node.to_string(), status_since - 1).await?;
        node_infos.insert(node_info.status_since, node_info.clone());
        status_since = node_info.status_since;
    }

    println!("\n{:#?}",node_infos);
    Ok(())
}

async fn getblock(node: String, height: u32) -> Result<NodeInfo, anyhow::Error> {
    let url = format!("{}/{}?height={}", BASE_URL,node,height);
    println!("url: {}", url);

    let resp = reqwest::get(url).await?.json::<NodeInfo>().await?;
    println!("{:#?}", resp);
    Ok(resp)
}
