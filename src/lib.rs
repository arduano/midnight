use std::{
    fmt::{Display, Formatter},
    future::Future,
};

use futures::StreamExt;
use thiserror::Error;
use twilight_gateway::{Cluster, Event, Intents};
use twilight_http::{response::DeserializeBodyError, Client, Error};
use twilight_model::{
    channel::Channel,
    id::{marker::ChannelMarker, Id},
};

pub mod twilight;

struct DiscordClient {
    client: Client,
}

#[derive(Debug, Error)]
enum DiscordError {
    TwilightError(#[from] Error),
    DeserializeError(#[from] DeserializeBodyError),
}

impl Display for DiscordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwilightError(e) => write!(f, "Twilight error: {}", e),
        }
    }
}

impl DiscordClient {
    pub fn new(token: String) -> Self {
        let client = Client::new(token);
        Self { client }
    }

    pub async fn get_channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> impl Future<Output = Result<Channel, DiscordError>> {
        let channel = self
            .client
            .channel(channel_id)
            .exec()
            .await?
            .model()
            .await?;
        println!("Channel: {:?}", channel);
    }
}

pub async fn run_socket_event_cluster<
    H: 'static + Fn(Event) -> F,
    F: 'static + Future<Output = ()>,
>(
    token: String,
    handler: H,
) {
    let intents = Intents::GUILDS | Intents::GUILD_MESSAGE_REACTIONS;

    let (cluster, mut events) = Cluster::builder(token, intents)
        .build()
        .await
        .expect("Failed to create cluster");

    tokio::spawn(async move {
        cluster.up().await;
    });

    while let Some((_, event)) = events.next().await {
        handler(event).await;
    }
}
