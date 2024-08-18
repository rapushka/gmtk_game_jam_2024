use crate::gameplay::cards::{Card, DeckRoot, EnemyCard, PlayerCard};
use crate::gameplay::cards::types::CardType;
use crate::prelude::*;
use crate::view::ui::create;

#[derive(Event)]
pub struct SpawnCard {
    pub card_type: CardType,
    pub parent: Entity,
    pub is_player_card: bool,
}

pub fn spawn_card(
    trigger: Trigger<SpawnCard>,
    mut commands: Commands,
    assets: Res<UiAssets>,
) {
    let event = trigger.event();
    let name = event.card_type.name();

    let entity_command = commands
        .spawn_with_name(&format!("card: {}", name))
        .insert(Card(event.card_type))
        .insert(NodeBundle {
            background_color: colors::card_background_color().into(),
            style: Style {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,

                padding: UiRect::all(Val::Px(5.0)),
                margin: UiRect::all(Val::Px(5.0)),

                ..default()
            },
            ..default()
        })
        .with_children(|item| {
            create::light_text(assets.font.clone(), name, item, view::CARD_NAME_FONT_SIZE)
        });

    if event.is_player_card {
        entity_command.insert(PlayerCard);
    } else {
        entity_command.insert(EnemyCard);
    }
    let entity = entity_command.id();

    commands.entity(event.parent).add_child(entity);
}