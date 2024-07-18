use crate::game::animation::BasicAnimation;
use crate::game::assets::{SpriteSheetAsset, SpriteSheetAssets};
use crate::game::movement::Velocity;
use bevy::prelude::*;
use bevy::sprite::SpriteAssetEvents;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_projectile);
    app.register_type::<Projectile>();
}

#[derive(Event, Debug)]
pub struct SpawnProjectile(pub Transform);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub struct Projectile;

fn spawn_projectile(
    trigger: Trigger<SpawnProjectile>,
    mut commands: Commands,
    sprite_sheets: Res<SpriteSheetAssets>,
) {
    let projectile_speed = 500.0;
    let mut trigger_point = trigger.event().0.clone();
    println!("Spawn projectile @ {:?}", trigger_point.translation);
    let direction = trigger_point.local_y().as_vec3().normalize() * projectile_speed;
    let offset = Vec3::new(0.0, 30.0, 0.0);
    trigger_point.translation += offset;
    let (texture, layout) = sprite_sheets[&SpriteSheetAsset::EnergyBall].clone();
    commands.spawn((
        Projectile,
        SpriteBundle {
            texture,
            transform: trigger_point,
            ..default()
        },
        TextureAtlas { layout, index: 0 },
        BasicAnimation::new(4, Duration::from_secs_f32(0.2)),
        Velocity::new(direction),
    ));
}
