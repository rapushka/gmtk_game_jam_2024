use crate::prelude::*;

#[derive(Event)]
pub struct Clicked;

pub fn visualise_interaction_with_buttons(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut background_color) in buttons.iter_mut() {
        let color = match *interaction {
            Interaction::Pressed => colors::pressed_button().into(),
            Interaction::Hovered => colors::hovered_button().into(),
            Interaction::None => colors::default_button().into(),
        };

        *background_color = color;
    }
}

pub fn click_on_pressed_button(
    mut commands: Commands,
    mut buttons: Query<(Entity, &Interaction), Changed<Interaction>>,
) {
    for (entity, interaction) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                commands.trigger_targets(Clicked, entity);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        };
    }
}