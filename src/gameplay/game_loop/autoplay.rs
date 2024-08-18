use crate::prelude::*;
pub use buttons::*;

mod buttons;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AutoplayState {
    #[default]
    Paused,
    Play {
        repeat: bool,
    },
}

impl AutoplayState {
    pub fn is_playing() -> IsAutoPlaying {
        IsAutoPlaying
    }
}

pub fn reset_autoplay(
    prev_autoplay: Res<State<AutoplayState>>,
    mut next_autoplay: ResMut<NextState<AutoplayState>>,
) {
    if let AutoplayState::Play { repeat } = **prev_autoplay {
        if !repeat {
            next_autoplay.set(AutoplayState::Paused);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct IsAutoPlaying;

impl ComputedStates for IsAutoPlaying {
    type SourceStates = AutoplayState;

    fn compute(sources: AutoplayState) -> Option<Self> {
        match sources {
            AutoplayState::Play { .. } => Some(Self),
            _ => None,
        }
    }
}