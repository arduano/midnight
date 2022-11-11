use twilight_http::request::channel::message::UpdateMessage;
use twilight_model::{channel::{embed::Embed, Message}, http::attachment::Attachment};

use super::{DiscordError, DiscordOperation, InnerTwilightError, await_model};

pub struct UpdateMessageBuilder<'a> {
    create: UpdateMessage<'a>,
    operation: DiscordOperation,
}

impl<'a> UpdateMessageBuilder<'a> {
    pub(super) fn new(create: UpdateMessage<'a>, operation: DiscordOperation) -> Self {
        Self { create, operation }
    }

    fn map_create<Err: Into<InnerTwilightError>>(
        mut self,
        f: impl FnOnce(UpdateMessage<'a>) -> Result<UpdateMessage<'a>, Err>,
    ) -> Result<UpdateMessageBuilder<'a>, DiscordError> {
        self.create =
            f(self.create).map_err(|err| DiscordError::new(err.into(), self.operation))?;
        Ok(self)
    }

    pub fn content(self, content: Option<&'a str>) -> Result<Self, DiscordError> {
        self.map_create(|create| create.content(content))
    }

    pub fn embeds(self, embeds: Option<&'a [Embed]>) -> Result<Self, DiscordError> {
        self.map_create(|create| create.embeds(embeds))
    }

    pub fn attachments(self, attachments: &'a [Attachment]) -> Result<Self, DiscordError> {
        self.map_create(|create| create.attachments(attachments))
    }

    pub async fn exec(self) -> Result<Message, DiscordError> {
        DiscordError::guard(self.operation, await_model!(self.create) ).await
    }
}
