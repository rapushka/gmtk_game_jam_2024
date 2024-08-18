use crate::prelude::*;
pub use buttons::*;

mod buttons;

#[derive(Resource)]
pub enum Autoplay {
    Paused,
    Play {
        repeat: bool,
    },
}

pub fn reset_autoplay(
    mut playmode: ResMut<Autoplay>,
) {
    if let Autoplay::Play { repeat } = *playmode {
        if !repeat {
            *playmode = Autoplay::Paused;
        }
    }
}
