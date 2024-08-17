use crate::gameplay::cards::spawn::*;
use crate::prelude::*;

pub mod spawn;

#[derive(Component)]
pub struct DeckRoot;

#[derive(Component)]
pub struct Card;

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnCard>()

            .add_systems(OnEnter(AppState::in_gameplay()), test_cards_spawn.after(crate::view::ui::gameplay::spawn::spawn_gameplay_hud))

            .observe(spawn_card)
        ;
    }
}

fn test_cards_spawn(
    mut commands: Commands,
) {
    commands.trigger(SpawnCard { name: "attack".to_string() });
    commands.trigger(SpawnCard { name: "attack".to_string() });
    commands.trigger(SpawnCard { name: "attack".to_string() });
    commands.trigger(SpawnCard { name: "attack".to_string() });
    commands.trigger(SpawnCard { name: "attack".to_string() });

    info!("--- events sent")
}