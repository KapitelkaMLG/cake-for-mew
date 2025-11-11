use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_rand::prelude::*;

mod assets;
mod cake;
mod gamba;
mod ui;
mod util;

use assets::Assets;

use crate::{cake::CakePlugin, gamba::GambaPlugin, ui::UiPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            Light2dPlugin,
            EntropyPlugin::<WyRand>::default(),
            CakePlugin,
            GambaPlugin,
            UiPlugin,
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Cake)
                .load_collection::<Assets>(),
        )
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Light2d {
            ambient_light: AmbientLight2d::default(),
        },
    ));
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Cake,
    Gamba,
}
