use twilight_http::request::channel::message::CreateMessage;
use twilight_model::{
    channel::{embed::Embed, Message},
    http::attachment::Attachment,
};

use super::{await_model, DiscordError, DiscordOperation, InnerTwilightError};

pub struct CreateMessageBuilder<'a> {
    create: CreateMessage<'a>,
    operation: DiscordOperation,
}

impl<'a> CreateMessageBuilder<'a> {
    pub(super) fn new(create: CreateMessage<'a>, operation: DiscordOperation) -> Self {
        Self { create, operation }
    }

    fn map_create<Err: Into<InnerTwilightError>>(
        mut self,
        f: impl FnOnce(CreateMessage<'a>) -> Result<CreateMessage<'a>, Err>,
    ) -> Result<CreateMessageBuilder<'a>, DiscordError> {
        self.create =
            f(self.create).map_err(|err| DiscordError::new(err.into(), self.operation))?;
        Ok(self)
    }

    pub fn content(self, content: &'a str) -> Result<Self, DiscordError> {
        self.map_create(|create| create.content(content))
    }

    pub fn embeds(self, embeds: &'a [Embed]) -> Result<Self, DiscordError> {
        self.map_create(|create| create.embeds(embeds))
    }

    pub fn attachments(self, attachments: &'a [Attachment]) -> Result<Self, DiscordError> {
        self.map_create(|create| create.attachments(attachments))
    }

    pub async fn exec(self) -> Result<Message, DiscordError> {
        DiscordError::guard(self.operation, await_model!(self.create)).await
    }
}
