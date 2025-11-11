use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct Assets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 10, rows = 10))]
    pub texture_atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "textures.png")]
    pub textures: Handle<Image>,
    #[asset(path = "eating_sound.ogg")]
    pub eating_sound: Handle<AudioSource>,
}
