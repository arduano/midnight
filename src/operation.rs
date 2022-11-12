use std::fmt::Display;

use twilight_model::id::{marker::*, Id};

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
    DeleteMessage {
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

impl Display for DiscordOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetCurrentUser => write!(f, "Get current user"),
            Self::GetChannel { channel_id } => write!(f, "Get channel {}", channel_id),
            Self::GetGuild { guild_id } => write!(f, "Get guild {}", guild_id),
            Self::GetMessage {
                channel_id,
                message_id,
            } => write!(f, "Get message {} in channel {}", message_id, channel_id),
            Self::GetMember { guild_id, user_id } => {
                write!(f, "Get member {} in guild {}", user_id, guild_id)
            }
            Self::GetUser { user_id } => write!(f, "Get user {}", user_id),
            Self::CreateMessage { channel_id } => {
                write!(f, "Create message in channel {}", channel_id)
            }
            Self::UpdateMessage {
                channel_id,
                message_id,
            } => write!(f, "Update message {} in channel {}", message_id, channel_id),
            Self::DeleteMessage {
                channel_id,
                message_id,
            } => write!(f, "Delete message {} in channel {}", message_id, channel_id),
            Self::AddRole {
                guild_id,
                user_id,
                role_id,
            } => write!(
                f,
                "Add role {} to user {} in guild {}",
                role_id, user_id, guild_id
            ),
            Self::RemoveRole {
                guild_id,
                user_id,
                role_id,
            } => write!(
                f,
                "Remove role {} from user {} in guild {}",
                role_id, user_id, guild_id
            ),
            Self::AddReaction {
                channel_id,
                message_id,
            } => write!(
                f,
                "Add reaction to message {} in channel {}",
                message_id, channel_id
            ),
            Self::RemoveOwnReaction {
                channel_id,
                message_id,
            } => write!(
                f,
                "Remove own reaction from message {} in channel {}",
                message_id, channel_id
            ),
            Self::RemoveReaction {
                channel_id,
                message_id,
                user_id,
            } => write!(
                f,
                "Remove reaction from message {} in channel {} by user {}",
                message_id, channel_id, user_id
            ),
        }
    }
}
