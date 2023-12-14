use tokio::task::JoinHandle;

mod agent;
mod block_header;
mod blockchain;
mod coordinator;

/// The address to which agents can connect to
pub const ADDRESS: &str = "127.0.0.1:3000";

/// The amount of time the coordinate waits before sending the latest block to the agents
pub const BLOCK_TIME_IN_SECONDS: u64 = 15;

/// The amount of miners in the network
pub const NUMBER_OF_AGENTS: u64 = 5;

/// The target difficulty for the proof of work
pub const DIFFICULTY_TARGET: [u8; 32] = [
    0x00, 0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let coordinator = coordinator::Coordinator::new();

    let broadcast_coordinator = coordinator.clone();

    let coordinator_handle: JoinHandle<_> = tokio::spawn(async move {
        coordinator
            .run(ADDRESS)
            .await
            .expect("Cannot start the coordinator");
    });

    let broadcast_handle: JoinHandle<_> = tokio::spawn(async move {
        broadcast_coordinator
            .broadcast_new_blocks()
            .await;
    });

    let mut agent_handles = Vec::new();

    for _ in 1..=NUMBER_OF_AGENTS {
        let agent_handle: JoinHandle<_> = tokio::spawn(async move {
            agent::Agent.start(ADDRESS).await;
        });
        agent_handles.push(agent_handle);
    }

    coordinator_handle.await.expect("Coordinator task failed");

    broadcast_handle.await.expect("Broadcast task failed");

    for agent_handle in agent_handles {
        agent_handle.await.expect("Agent task failed");
    }
}
