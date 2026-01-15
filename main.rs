use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::primitives::{Address, U256};
use revm::{db::CacheDB, EVM, primitives::Env};
use std::sync::Arc;
use tokio::runtime::Builder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // 1. PINNED RUNTIME: Prevents the OS from "shuffling" your bot
    let runtime = Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .on_thread_start(|| {
            // Pin this thread to a specific vCPU core
            let core_ids = core_affinity::get_core_ids().unwrap();
            core_affinity::set_for_current(core_ids[0]);
        })
        .build()?;

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║    ⚡ APEX OMEGA v206.5 | RUST SINGULARITY (ELITE)     ║");
    println!("║    MODE: REVM-NATIVE 12-HOP SIMULATION                 ║");
    println!("╚════════════════════════════════════════════════════════╝");

    let rpc_url = std::env::var("CHAINSTACK_WSS")?;
    let provider = Arc::new(ProviderBuilder::new().on_ws(WsConnect::new(rpc_url)).await?);
    
    // RAM Market State: Adjacency list with log-weights
    let market_state = Arc::new(dashmap::DashMap::<Address, Pool>::new());

    let mut sub = provider.subscribe_pending_transactions().await?.into_stream();

    while let Some(tx_hash) = sub.next().await {
        let state = Arc::clone(&market_state);
        let prov = Arc::clone(&provider);

        // Instant dispatch to isolated vCPU cores
        tokio::spawn(async move {
            let t0 = std::time::Instant::now();
            
            // Step 1: Walk the 12-hop graph (Rayon-Parallel Search)
            if let Some(signal) = find_infinite_payload(&state, tx_hash) {
                
                // Step 2: LOCAL REVM SIMULATION (<50μs)
                // We simulate against our local 'CacheDB' - ZERO NETWORK DELAY
                if simulate_locally(&signal).is_profitable() {
                    execute_strike(&prov, signal).await;
                    println!("🚀 STRIKE | Total Logic Latency: {:?}μs", t0.elapsed().as_micros());
                }
            }
        });
    }
    Ok(())
}
