//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::game::assets::SpriteSheetAssets;
use crate::{
    game::assets::{ImageAssets, SoundtrackAssets},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(Update, check_all_loaded.run_if(in_state(Screen::Loading)));
}

fn enter_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });

    // Preload assets so the game runs smoothly.
    commands.insert_resource(ImageAssets::new(&asset_server));
    commands.insert_resource(SpriteSheetAssets::new(
        &asset_server,
        &mut texture_atlas_layouts,
    ));
    commands.insert_resource(SoundtrackAssets::new(&asset_server));
}

fn check_all_loaded(
    image_assets: Res<Assets<Image>>,
    audio_assets: Res<Assets<AudioSource>>,
    tex_atl_layout_assets: Res<Assets<TextureAtlasLayout>>,
    images: Res<ImageAssets>,
    sprite_sheets: Res<SpriteSheetAssets>,
    soundtracks: Res<SoundtrackAssets>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let all_loaded = images.all_loaded(&image_assets)
        && sprite_sheets.all_loaded(&image_assets, &tex_atl_layout_assets)
        && soundtracks.all_loaded(&audio_assets);
    if all_loaded {
        next_screen.set(Screen::Title);
    }
}
