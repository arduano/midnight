use std::fmt::{Display, Formatter};

use futures::Future;
use thiserror::Error;
use twilight_http::{
    request::channel::reaction::RequestReactionType, response::DeserializeBodyError, Client, Error,
};
use twilight_model::{
    channel::{Channel, Message},
    guild::{Guild, Member},
    id::{marker::*, Id},
    user::{CurrentUser, User},
};
use twilight_validate::message::MessageValidationError;

use self::{create_message::CreateMessageBuilder, update_message::UpdateMessageBuilder};

pub mod create_message;
pub mod update_message;

#[derive(Debug, Error)]
pub enum InnerTwilightError {
    TwilightError(#[from] Error),
    DeserializeError(#[from] DeserializeBodyError),
    ValidationError(#[from] MessageValidationError),
}

impl Display for InnerTwilightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwilightError(e) => write!(f, "Twilight error: {}", e),
            Self::DeserializeError(e) => write!(f, "Deserialize error: {}", e),
            Self::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DiscordOperation {
    GetCurrentUser,
    GetChannel {
        channel_id: Id<ChannelMarker>,
    },
    GetGuild {
        guild_id: Id<GuildMarker>,
    },
    GetMessage {
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    },
    GetMember {
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    },
    GetUser {
        user_id: Id<UserMarker>,
    },
    CreateMessage {
        channel_id: Id<ChannelMarker>,
    },
    UpdateMessage {
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    },
    AddRole {
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    },
    RemoveRole {
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    },
    AddReaction {
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    },
    RemoveOwnReaction {
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    },
    RemoveReaction {
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        user_id: Id<UserMarker>,
    },
}

impl DiscordError {
    fn new(inner: InnerTwilightError, operation: DiscordOperation) -> Self {
        Self { inner, operation }
    }

    fn guard<T, Fut: Future<Output = Result<T, InnerTwilightError>>>(
        operation: DiscordOperation,
        f: impl FnOnce() -> Fut,
    ) -> impl Future<Output = Result<T, DiscordError>> {
        async move {
            f().await
                .map_err(|e| DiscordError::new(e.into(), operation))
        }
    }
}

pub struct DiscordError {
    pub inner: InnerTwilightError,
    pub operation: DiscordOperation,
}

pub struct DiscordClient {
    client: Client,
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

impl DiscordClient {
    pub fn new(token: String) -> Self {
        let client = Client::new(token);
        Self { client }
    }

    pub async fn current_user(&self) -> Result<CurrentUser, DiscordError> {
        DiscordError::guard(
            DiscordOperation::GetCurrentUser,
            await_model!(self.client.current_user()),
        )
        .await
    }

    pub async fn channel(&self, channel_id: Id<ChannelMarker>) -> Result<Channel, DiscordError> {
        DiscordError::guard(
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
        DiscordError::guard(
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
        DiscordError::guard(
            DiscordOperation::GetMember { guild_id, user_id },
            await_model!(self.client.guild_member(guild_id, user_id)),
        )
        .await
    }

    pub async fn guild(&self, guild_id: Id<GuildMarker>) -> Result<Guild, DiscordError> {
        DiscordError::guard(
            DiscordOperation::GetGuild { guild_id },
            await_model!(self.client.guild(guild_id)),
        )
        .await
    }

    pub async fn user(&self, user_id: Id<UserMarker>) -> Result<User, DiscordError> {
        DiscordError::guard(
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
        DiscordError::guard(
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
        DiscordError::guard(
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
        DiscordError::guard(
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
        DiscordError::guard(
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
        DiscordError::guard(
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
