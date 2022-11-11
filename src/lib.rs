use std::future::Future;

use crate::twilight::gateway::{Cluster, Event, Intents};

use futures::StreamExt;
use twilight_gateway::cluster::ClusterStartError;

pub mod client;
pub mod twilight;

pub async fn run_discord_event_loop<
    H: 'static + Fn(Event) -> F,
    F: 'static + Future<Output = ()>,
>(
    token: String,
    handler: H,
    intents: Intents,
) -> Result<(), ClusterStartError> {
    let (cluster, mut events) = Cluster::builder(token, intents).build().await?;

    tokio::spawn(async move {
        cluster.up().await;
    });

    while let Some((_, event)) = events.next().await {
        handler(event).await;
    }

    Ok(())
}

pub async fn run_discord_event_loop_or_panic<
    H: 'static + Fn(Event) -> F,
    F: 'static + Future<Output = ()>,
>(
    token: String,
    handler: H,
    intents: Intents,
) {
    run_discord_event_loop(token, handler, intents)
        .await
        .expect("Failed to start Discord event loop");
}
