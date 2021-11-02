pub mod producer;

use crate::{
	audio_stream::{AudioStreamId, AudioStreamWrapper},
	clock::{Clock, ClockId},
	parameter::{Parameter, ParameterId, Tween},
	track::{SubTrackId, Track, TrackId},
	value::Value,
};

pub(crate) enum ParameterCommand {
	Add(ParameterId, Parameter),
	Set {
		id: ParameterId,
		target: f64,
		tween: Tween,
	},
	Pause(ParameterId),
	Resume(ParameterId),
}

pub(crate) enum MixerCommand {
	AddSubTrack(SubTrackId, Track),
	SetTrackVolume(TrackId, Value),
	SetTrackPanning(TrackId, Value),
}

pub(crate) enum ClockCommand {
	Add(ClockId, Clock),
	SetInterval(ClockId, Value),
	Start(ClockId),
	Pause(ClockId),
	Stop(ClockId),
}

pub(crate) enum AudioStreamCommand {
	Add(AudioStreamId, AudioStreamWrapper),
}

pub(crate) enum Command {
	Parameter(ParameterCommand),
	Mixer(MixerCommand),
	Clock(ClockCommand),
	AudioStream(AudioStreamCommand),
	Pause(Tween),
	Resume(Tween),
}
