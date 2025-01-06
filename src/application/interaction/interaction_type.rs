use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Type of interaction.
///
/// See [Discord Docs/Interaction Object].
///
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum InteractionType {
    /// Interaction involves a ping (webhook-based interactions).
    ///
    /// See [Discord Docs/Receiving an Interaction].
    ///
    /// [Discord Docs/Receiving an Interaction]: https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction
    Ping = 1,
    /// Interaction involves an application command.
    ApplicationCommand = 2,
    /// Interaction involves a message [`Component`].
    ///
    /// [`Component`]: crate::channel::message::Component
    MessageComponent = 3,
    /// Interaction involves an autocomplete request.
    ApplicationCommandAutocomplete = 4,
    /// Interaction involves a modal submit.
    ModalSubmit = 5,
}

impl InteractionType {
    pub const fn kind(self) -> &'static str {
        match self {
            Self::Ping => "Ping",
            Self::ApplicationCommand => "ApplicationCommand",
            Self::MessageComponent => "MessageComponent",
            Self::ApplicationCommandAutocomplete => "ApplicationCommandAutocomplete",
            Self::ModalSubmit => "ModalSubmit",
        }
    }
}

#[derive(Debug)]
pub struct UnknownInteractionTypeError {
    value: u8,
}

impl Display for UnknownInteractionTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("unknown interaction type: ")?;

        Display::fmt(&self.value, f)
    }
}

impl TryFrom<u8> for InteractionType {
    type Error = UnknownInteractionTypeError;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Ping),
            2 => Ok(Self::ApplicationCommand),
            3 => Ok(Self::MessageComponent),
            4 => Ok(Self::ApplicationCommandAutocomplete),
            5 => Ok(Self::ModalSubmit),
            other => Err(UnknownInteractionTypeError { value: other }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InteractionType, UnknownInteractionTypeError};
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        InteractionType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );
    const_assert_eq!(1, InteractionType::Ping as u8);
    const_assert_eq!(2, InteractionType::ApplicationCommand as u8);
    const_assert_eq!(3, InteractionType::MessageComponent as u8);
    const_assert_eq!(4, InteractionType::ApplicationCommandAutocomplete as u8);
    const_assert_eq!(5, InteractionType::ModalSubmit as u8);

    #[test]
    fn kind() {
        assert_eq!("Ping", InteractionType::Ping.kind());
        assert_eq!(
            "ApplicationCommand",
            InteractionType::ApplicationCommand.kind()
        );
        assert_eq!("MessageComponent", InteractionType::MessageComponent.kind());
        assert_eq!(
            "ApplicationCommandAutocomplete",
            InteractionType::ApplicationCommandAutocomplete.kind()
        );
        assert_eq!("ModalSubmit", InteractionType::ModalSubmit.kind());
    }

    #[test]
    fn try_from() -> Result<(), UnknownInteractionTypeError> {
        assert_eq!(InteractionType::Ping, InteractionType::try_from(1)?);
        assert_eq!(
            InteractionType::ApplicationCommand,
            InteractionType::try_from(2)?
        );
        assert_eq!(
            InteractionType::MessageComponent,
            InteractionType::try_from(3)?
        );
        assert_eq!(
            InteractionType::ApplicationCommandAutocomplete,
            InteractionType::try_from(4)?,
        );
        assert_eq!(InteractionType::ModalSubmit, InteractionType::try_from(5)?);
        assert!(InteractionType::try_from(u8::MAX).is_err());

        Ok(())
    }
}
