use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct Assets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 10, rows = 10))]
    pub texture_atlas: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 32, columns = 2, rows = 4))]
    pub pond_atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "textures.png")]
    pub textures: Handle<Image>,
    #[asset(path = "pond.png")]
    pub pond: Handle<Image>,
    #[asset(path = "eating_sound.ogg")]
    pub eating_sound: Handle<AudioSource>,
    #[asset(path = "pickle_mew.ogg")]
    pub pickle_mew: Handle<AudioSource>,
    #[asset(path = "win1.ogg")]
    pub win1: Handle<AudioSource>,
    #[asset(path = "win2.ogg")]
    pub win2: Handle<AudioSource>,
    #[asset(path = "win3.ogg")]
    pub win3: Handle<AudioSource>,
    #[asset(path = "loss1.ogg")]
    pub loss1: Handle<AudioSource>,
    #[asset(path = "loss2.ogg")]
    pub loss2: Handle<AudioSource>,
    #[asset(path = "loss3.ogg")]
    pub loss3: Handle<AudioSource>,
    #[asset(path = "loss4.ogg")]
    pub loss4: Handle<AudioSource>,
    #[asset(path = "loss5.ogg")]
    pub loss5: Handle<AudioSource>,
    #[asset(path = "loss6.ogg")]
    pub loss6: Handle<AudioSource>,
    #[asset(path = "loss7.ogg")]
    pub loss7: Handle<AudioSource>,
    #[asset(path = "loss8.ogg")]
    pub loss8: Handle<AudioSource>,
    #[asset(path = "loss9.ogg")]
    pub loss9: Handle<AudioSource>,
    #[asset(path = "bankrupt1.ogg")]
    pub bankrupt1: Handle<AudioSource>,
    #[asset(path = "bankrupt2.ogg")]
    pub bankrupt2: Handle<AudioSource>,
    #[asset(path = "bankrupt3.ogg")]
    pub bankrupt3: Handle<AudioSource>,
    #[asset(path = "bankrupt4.ogg")]
    pub bankrupt4: Handle<AudioSource>,
}
