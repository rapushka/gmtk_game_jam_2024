use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Bootstrap)
                    .load_collection::<UiAssets>()
                    .load_collection::<CharacterAssets>()
                    .load_collection::<EnemyAssets>()
                    .continue_to_state(AppState::MainMenu)
            )
        ;
    }
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/rounded_square.png")]
    pub rounded_square: Handle<Image>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "ui/play.png")]
    pub icon_play: Handle<Image>,

    #[asset(path = "ui/pause.png")]
    pub icon_pause: Handle<Image>,

    #[asset(path = "ui/step.png")]
    pub icon_step: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(path = "characters/bouncer.png")]
    pub bouncer: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct EnemyAssets {
    #[asset(path = "enemies/rat.png")]
    pub rat: Handle<Image>,
}
