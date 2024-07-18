//! Spawn the player.

use bevy::prelude::*;

use crate::game::assets::{SpriteSheetAsset, SpriteSheetAssets};
use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{ImageAsset, ImageAssets},
        movement::{Movement, MovementController, WrapWithinWindow},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    sprite_sheets: Res<SpriteSheetAssets>,
) {
    let (texture, texture_atlas_layout) = sprite_sheets[&SpriteSheetAsset::Spaceship].clone();
    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture,
            transform: Transform::from_scale(Vec2::splat(1.0).extend(1.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        },
        MovementController::default(),
        Movement {
            thrust_speed: 320.0,
            rotate_speed: 5.0,
        },
        WrapWithinWindow,
        PlayerAnimation::new(),
        StateScoped(Screen::Playing),
    ));
}
