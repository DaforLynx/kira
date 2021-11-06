//! Errors that can occur when using an [`AudioManager`](super::AudioManager).

use std::{
	error::Error,
	fmt::{Display, Formatter},
};

use crate::error::CommandError;

/// Errors that can occur when playing a sound.
#[derive(Debug)]
pub enum PlaySoundError {
	/// Could not play a sound because the maximum number of sounds has been reached.
	SoundLimitReached,
	/// An error occured when sending a command to the renderer.
	CommandError(CommandError),
}

impl Display for PlaySoundError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			PlaySoundError::SoundLimitReached => f.write_str(
				"Could not play a sound because the maximum number of sounds has been reached.",
			),
			PlaySoundError::CommandError(error) => error.fmt(f),
		}
	}
}

impl Error for PlaySoundError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			PlaySoundError::CommandError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<CommandError> for PlaySoundError {
	fn from(v: CommandError) -> Self {
		Self::CommandError(v)
	}
}

/// Errors that can occur when creating a parameter.
#[derive(Debug)]
pub enum AddParameterError {
	/// Could not add a parameter because the maximum number of parameters has been reached.
	ParameterLimitReached,
	/// An error occured when sending a command to the renderer.
	CommandError(CommandError),
}

impl Display for AddParameterError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
            AddParameterError::ParameterLimitReached => f.write_str("Could not add a parameter because the maximum number of parameters has been reached."),
            AddParameterError::CommandError(error) => error.fmt(f),
        }
	}
}

impl Error for AddParameterError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			AddParameterError::CommandError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<CommandError> for AddParameterError {
	fn from(v: CommandError) -> Self {
		Self::CommandError(v)
	}
}

/// Errors that can occur when creating a mixer sub-track.
#[derive(Debug)]
pub enum AddSubTrackError {
	/// Could not add a sub-track because the maximum number of sub-tracks has been reached.
	SubTrackLimitReached,
	/// An error occured when sending a command to the renderer.
	CommandError(CommandError),
}

impl Display for AddSubTrackError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			AddSubTrackError::SubTrackLimitReached => f.write_str("Could not add a sub-track because the maximum number of sub-tracks has been reached."),
			AddSubTrackError::CommandError(error) => error.fmt(f),
		}
	}
}

impl Error for AddSubTrackError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			AddSubTrackError::CommandError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<CommandError> for AddSubTrackError {
	fn from(v: CommandError) -> Self {
		Self::CommandError(v)
	}
}

/// Errors that can occur when creating a clock.
#[derive(Debug)]
pub enum AddClockError {
	/// Could not add a clock because the maximum number of clocks has been reached.
	ClockLimitReached,
	/// An error occured when sending a command to the renderer.
	CommandError(CommandError),
}

impl Display for AddClockError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			AddClockError::ClockLimitReached => f.write_str(
				"Could not add a clock because the maximum number of clocks has been reached.",
			),
			AddClockError::CommandError(error) => error.fmt(f),
		}
	}
}

impl Error for AddClockError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			AddClockError::CommandError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<CommandError> for AddClockError {
	fn from(v: CommandError) -> Self {
		Self::CommandError(v)
	}
}

/// Errors that can occur when creating a audio stream.
#[derive(Debug)]
pub enum AddAudioStreamError {
	/// Could not add a audio stream because the maximum number of audio streams has been reached.
	AudioStreamLimitReached,
	/// An error occured when sending a command to the renderer.
	CommandError(CommandError),
}

impl Display for AddAudioStreamError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			AddAudioStreamError::AudioStreamLimitReached => f.write_str("Could not add a audio stream because the maximum number of audio streams has been reached."),
			AddAudioStreamError::CommandError(error) => error.fmt(f),
		}
	}
}

impl Error for AddAudioStreamError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			AddAudioStreamError::CommandError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<CommandError> for AddAudioStreamError {
	fn from(v: CommandError) -> Self {
		Self::CommandError(v)
	}
}
