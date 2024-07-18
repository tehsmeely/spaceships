pub mod soundtrack;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, button_interaction_sfx);

    app.observe(soundtrack::play_soundtrack);
}

fn button_interaction_sfx(
    mut interactions: Query<&'static Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => (),
            Interaction::Pressed => (),
            _ => {}
        }
    }
}
