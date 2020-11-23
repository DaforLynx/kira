use std::{
	hash::Hash,
	sync::atomic::{AtomicUsize, Ordering},
};

use crate::{mixer::TrackIndex, playable::Metadata};

static NEXT_SOUND_INDEX: AtomicUsize = AtomicUsize::new(0);

/// A unique identifier for a [`Sound`](crate::sound::Sound).
///
/// You cannot create this manually - a sound ID is returned
/// when you load a sound with an [`AudioManager`](crate::manager::AudioManager).
#[derive(Debug, Copy, Clone)]
pub struct SoundId {
	index: usize,
	duration: f64,
	default_track_index: TrackIndex,
	metadata: Metadata,
}

impl SoundId {
	pub fn duration(&self) -> f64 {
		self.duration
	}

	pub fn default_track_index(&self) -> TrackIndex {
		self.default_track_index
	}

	pub fn metadata(&self) -> &Metadata {
		&self.metadata
	}
}

impl PartialEq for SoundId {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

impl Eq for SoundId {}

impl Hash for SoundId {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.index.hash(state);
	}
}

impl SoundId {
	pub(crate) fn new(duration: f64, default_track_index: TrackIndex, metadata: Metadata) -> Self {
		let index = NEXT_SOUND_INDEX.fetch_add(1, Ordering::Relaxed);
		Self {
			index,
			duration,
			default_track_index,
			metadata,
		}
	}
}
