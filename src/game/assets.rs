use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum ImageAsset {
    Ducky,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct ImageAssets(HashMap<ImageAsset, Handle<Image>>);

impl ImageAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut assets = HashMap::new();

        assets.insert(
            ImageAsset::Ducky,
            asset_server.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        );

        Self(assets)
    }

    pub fn all_loaded(&self, assets: &Assets<Image>) -> bool {
        self.0.iter().all(|(_, handle)| assets.contains(handle))
    }
}

#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SpriteSheetAsset {
    Spaceship,
    EnergyBall,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct SpriteSheetAssets(
    HashMap<SpriteSheetAsset, (Handle<Image>, Handle<TextureAtlasLayout>)>,
);

impl SpriteSheetAssets {
    pub fn new(
        asset_server: &AssetServer,
        mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let mut assets = HashMap::new();

        let spaceship_handle = asset_server.load_with_settings(
            "images/ship.png",
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            },
        );
        let spaceship_atlas = TextureAtlasLayout::from_grid(UVec2::splat(70), 3, 1, None, None);
        let spaceship_atlas_handle = texture_atlas_layouts.add(spaceship_atlas);
        assets.insert(
            SpriteSheetAsset::Spaceship,
            (spaceship_handle, spaceship_atlas_handle),
        );

        let energy_ball_handle = asset_server.load_with_settings(
            "images/energy_ball.png",
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            },
        );
        let energy_ball_atlas = TextureAtlasLayout::from_grid(UVec2::splat(40), 4, 1, None, None);
        let energy_ball_atlas_handle = texture_atlas_layouts.add(energy_ball_atlas);
        assets.insert(
            SpriteSheetAsset::EnergyBall,
            (energy_ball_handle, energy_ball_atlas_handle),
        );
        Self(assets)
    }

    pub fn all_loaded(
        &self,
        images: &Assets<Image>,
        atlas_layouts: &Assets<TextureAtlasLayout>,
    ) -> bool {
        self.0.iter().all(|(_, (img_handle, tal_handle))| {
            images.contains(img_handle) && atlas_layouts.contains(tal_handle)
        })
    }
}
#[derive(PartialEq, Eq, Hash, Reflect)]
pub enum SoundtrackAsset {
    Credits,
    Gameplay,
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct SoundtrackAssets(HashMap<SoundtrackAsset, Handle<AudioSource>>);

impl SoundtrackAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut assets = HashMap::new();
        assets.insert(
            SoundtrackAsset::Credits,
            asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
        );
        assets.insert(
            SoundtrackAsset::Gameplay,
            asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
        );
        Self(assets)
    }

    pub fn all_loaded(&self, assets: &Assets<AudioSource>) -> bool {
        self.0.iter().all(|(_, handle)| assets.contains(handle))
    }
}
