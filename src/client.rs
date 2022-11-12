use std::sync::Arc;

use futures::Future;

use twilight_http::{request::channel::reaction::RequestReactionType, Client};
use twilight_model::{
    channel::{Channel, Message},
    guild::{Guild, Member},
    id::{marker::*, Id},
    user::{CurrentUser, User},
};

use crate::{
    error::{DiscordError, InnerTwilightError},
    operation::DiscordOperation,
};

use self::{create_message::CreateMessageBuilder, update_message::UpdateMessageBuilder};

pub mod create_message;
pub mod update_message;

#[derive(Clone)]
pub struct DiscordClient {
    client: Arc<Client>,
}

macro_rules! await_model {
    ($value:expr) => {
        || async move { Ok($value.exec().await?.model().await?) }
    };
}

macro_rules! await_empty {
    ($value:expr) => {
        || async move {
            $value.exec().await?;
            Ok(())
        }
    };
}

pub(self) use await_model;

fn error_guard<T, Fut: Future<Output = Result<T, InnerTwilightError>>>(
    operation: DiscordOperation,
    f: impl FnOnce() -> Fut,
) -> impl Future<Output = Result<T, DiscordError>> {
    async move {
        f().await
            .map_err(|e| DiscordError::new(e.into(), operation))
    }
}

impl DiscordClient {
    pub fn new(token: String) -> Self {
        let client = Client::new(token);
        Self {
            client: Arc::new(client),
        }
    }

    pub async fn current_user(&self) -> Result<CurrentUser, DiscordError> {
        error_guard(
            DiscordOperation::GetCurrentUser,
            await_model!(self.client.current_user()),
        )
        .await
    }

    pub async fn channel(&self, channel_id: Id<ChannelMarker>) -> Result<Channel, DiscordError> {
        error_guard(
            DiscordOperation::GetChannel { channel_id },
            await_model!(self.client.channel(channel_id)),
        )
        .await
    }

    pub async fn message(
        &self,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Result<Message, DiscordError> {
        error_guard(
            DiscordOperation::GetMessage {
                channel_id,
                message_id,
            },
            await_model!(self.client.message(channel_id, message_id)),
        )
        .await
    }

    pub async fn member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<Member, DiscordError> {
        error_guard(
            DiscordOperation::GetMember { guild_id, user_id },
            await_model!(self.client.guild_member(guild_id, user_id)),
        )
        .await
    }

    pub async fn guild(&self, guild_id: Id<GuildMarker>) -> Result<Guild, DiscordError> {
        error_guard(
            DiscordOperation::GetGuild { guild_id },
            await_model!(self.client.guild(guild_id)),
        )
        .await
    }

    pub async fn user(&self, user_id: Id<UserMarker>) -> Result<User, DiscordError> {
        error_guard(
            DiscordOperation::GetUser { user_id },
            await_model!(self.client.user(user_id)),
        )
        .await
    }

    pub fn create_message(&self, channel_id: Id<ChannelMarker>) -> CreateMessageBuilder {
        CreateMessageBuilder::new(
            self.client.create_message(channel_id),
            DiscordOperation::CreateMessage { channel_id },
        )
    }

    pub fn update_message(
        &self,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> UpdateMessageBuilder {
        UpdateMessageBuilder::new(
            self.client.update_message(channel_id, message_id),
            DiscordOperation::UpdateMessage {
                channel_id,
                message_id,
            },
        )
    }

    pub async fn add_role(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), DiscordError> {
        error_guard(
            DiscordOperation::AddRole {
                guild_id,
                user_id,
                role_id,
            },
            await_empty!(self
                .client
                .add_guild_member_role(guild_id, user_id, role_id)),
        )
        .await
    }

    pub async fn remove_role(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), DiscordError> {
        error_guard(
            DiscordOperation::RemoveRole {
                guild_id,
                user_id,
                role_id,
            },
            await_empty!(self
                .client
                .remove_guild_member_role(guild_id, user_id, role_id)),
        )
        .await
    }

    pub async fn add_reaction(
        &self,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        reaction: &RequestReactionType<'_>,
    ) -> Result<(), DiscordError> {
        error_guard(
            DiscordOperation::AddReaction {
                channel_id,
                message_id,
            },
            await_empty!(self
                .client
                .create_reaction(channel_id, message_id, reaction)),
        )
        .await
    }

    pub async fn remove_own_reaction(
        &self,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        reaction: &RequestReactionType<'_>,
    ) -> Result<(), DiscordError> {
        error_guard(
            DiscordOperation::RemoveOwnReaction {
                channel_id,
                message_id,
            },
            await_empty!(self
                .client
                .delete_current_user_reaction(channel_id, message_id, reaction)),
        )
        .await
    }

    pub async fn remove_reaction(
        &self,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        reaction: &RequestReactionType<'_>,
        user_id: Id<UserMarker>,
    ) -> Result<(), DiscordError> {
        error_guard(
            DiscordOperation::RemoveReaction {
                channel_id,
                message_id,
                user_id,
            },
            await_empty!(self
                .client
                .delete_reaction(channel_id, message_id, reaction, user_id)),
        )
        .await
    }
}
