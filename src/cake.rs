use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;

use crate::{
    GameState,
    assets::Assets,
    util::{Animation, animate, despawn_all},
};

pub struct CakePlugin;

impl Plugin for CakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Cake), (setup, spawn_amogi))
            .add_systems(
                Update,
                (animate::<FlameAnimation>, animate::<SmokeAnimation>)
                    .run_if(in_state(GameState::Cake)),
            )
            .add_systems(OnExit(GameState::Cake), despawn_all::<OnCakeScreen>);
    }
}

#[derive(Component)]
struct OnCakeScreen;

const CAKE_ATLAS_INDEX: usize = 0;
const CAKE_ATLAS_SIZE: usize = 10;
const PLATE_ATLAS_INDEX: usize = 10;
const PICKLE_MEW_ATLAS_INDEX: usize = 70;

const CAKE_SCALE: Vec3 = Vec3::splat(20.);
const AMOGUS_SCALE: Vec3 = Vec3::splat(3.5);

fn setup(
    mut commands: Commands,
    assets: Res<Assets>,
    mut light_query: Query<&mut Light2d, With<Camera>>,
    mut camera_transform: Single<&mut Transform, With<Camera>>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    camera_transform.translation.y = 0.;

    commands.insert_resource(ClearColor(Color::srgb_u8(29, 29, 29)));
    for mut light in &mut light_query {
        light.ambient_light.brightness = 0.3;
    }

    if rng.next_u32().is_multiple_of(5) {
        commands
            .spawn((
                Sprite::from_atlas_image(
                    assets.textures.clone(),
                    TextureAtlas::from(assets.texture_atlas.clone())
                        .with_index(PICKLE_MEW_ATLAS_INDEX),
                ),
                Transform::from_xyz(-300., -300., 1.),
                OnCakeScreen,
                PickleMew,
                Pickable::default(),
            ))
            .observe(click_pickle_mew);
    }

    commands.spawn((
        Sprite::from_atlas_image(
            assets.textures.clone(),
            TextureAtlas::from(assets.texture_atlas.clone()).with_index(PLATE_ATLAS_INDEX),
        ),
        Transform::from_xyz(0., 0., -1.).with_scale(CAKE_SCALE),
        OnCakeScreen,
    ));

    commands
        .spawn((
            Sprite::from_atlas_image(
                assets.textures.clone(),
                TextureAtlas::from(assets.texture_atlas.clone()).with_index(CAKE_ATLAS_INDEX),
            ),
            Transform::from_xyz(0., 0., 0.).with_scale(CAKE_SCALE),
            OnCakeScreen,
            Pickable::default(),
        ))
        .observe(bite_cake);
}

fn spawn_amogi(
    mut commands: Commands,
    assets: Res<Assets>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    spawn_amogus::<Amogus1>(&mut commands, &mut rng, &assets, Vec2::new(140., 120.));
    spawn_amogus::<Amogus2>(&mut commands, &mut rng, &assets, Vec2::new(-160., 110.));
    spawn_amogus::<Amogus3>(&mut commands, &mut rng, &assets, Vec2::new(80., 90.));
    spawn_amogus::<Amogus4>(&mut commands, &mut rng, &assets, Vec2::new(-80., 100.));
    spawn_amogus::<Amogus5>(&mut commands, &mut rng, &assets, Vec2::new(0., 140.));
}

fn spawn_amogus<A: Amogus + Component>(
    commands: &mut Commands,
    rng: &mut WyRand,
    assets: &Assets,
    pos: Vec2,
) {
    let index = rng.next_u32() as usize % A::size();
    let flip = rng.next_u32().is_multiple_of(2);
    commands
        .spawn((
            Sprite {
                image: assets.textures.clone(),
                texture_atlas: Some(
                    TextureAtlas::from(assets.texture_atlas.clone()).with_index(A::index() + index),
                ),
                flip_x: flip,
                ..default()
            },
            A::new(),
            OnCakeScreen,
            Transform::from_translation(pos.extend(2.)).with_scale(AMOGUS_SCALE),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Sprite {
                        image: assets.textures.clone(),
                        texture_atlas: Some(
                            TextureAtlas::from(assets.texture_atlas.clone())
                                .with_index(FlameAnimation::index()),
                        ),
                        flip_x: flip,
                        ..default()
                    },
                    PointLight2d {
                        radius: 120.,
                        color: Color::srgba_u8(200, 200, 0, 128),
                        intensity: 2.,
                        falloff: 10.,
                        ..default()
                    },
                    Pickable::default(),
                    FlameAnimation(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    Transform::from_xyz(0., 0., 3.),
                ))
                .observe(extinguish_flame);
        });
}

fn bite_cake(
    event: On<Pointer<Press>>,
    mut commands: Commands,
    mut sprites: Query<&mut Sprite>,
    assets: Res<Assets>,
) {
    if let Ok(mut sprite) = sprites.get_mut(event.event_target())
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        match atlas.index {
            v if v == CAKE_ATLAS_INDEX => {
                let id = commands.register_system(despawn_all::<Amogus1>);
                commands.run_system(id);
            }
            v if v == CAKE_ATLAS_INDEX + 1 => {
                let id = commands.register_system(despawn_all::<Amogus2>);
                commands.run_system(id);
            }
            v if v == CAKE_ATLAS_INDEX + 3 => {
                let id = commands.register_system(despawn_all::<Amogus4>);
                commands.run_system(id);

                let id = commands.register_system(despawn_all::<Amogus5>);
                commands.run_system(id);
            }
            v if v == CAKE_ATLAS_INDEX + 4 => {
                let id = commands.register_system(despawn_all::<Amogus3>);
                commands.run_system(id);
            }
            v if v == CAKE_ATLAS_INDEX + CAKE_ATLAS_SIZE - 1 => {
                let id = commands.register_system(spawn_amogi);
                commands.run_system(id);
            }
            _ => (),
        }
        if atlas.index != CAKE_ATLAS_INDEX + CAKE_ATLAS_SIZE - 1 {
            commands.spawn(AudioPlayer(assets.eating_sound.clone()));
        }

        atlas.index = (atlas.index + 1 - CAKE_ATLAS_INDEX) % CAKE_ATLAS_SIZE + CAKE_ATLAS_INDEX;
    }
}

fn extinguish_flame(
    event: On<Pointer<Press>>,
    mut commands: Commands,
    mut query: Query<(&mut Sprite, &mut PointLight2d)>,
) {
    if let Ok((mut sprite, mut light)) = query.get_mut(event.event_target())
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        light.radius = 0.;
        commands
            .entity(event.event_target())
            .remove::<Pickable>()
            .remove::<FlameAnimation>()
            .insert(SmokeAnimation(Timer::from_seconds(
                0.3,
                TimerMode::Repeating,
            )));
        atlas.index = SmokeAnimation::index();
    }
}

#[derive(Component)]
struct Amogus1;
#[derive(Component)]
struct Amogus2;
#[derive(Component)]
struct Amogus3;
#[derive(Component)]
struct Amogus4;
#[derive(Component)]
struct Amogus5;

#[derive(Component)]
struct PickleMew;

trait Amogus {
    fn index() -> usize {
        20
    }
    fn size() -> usize {
        5
    }
    fn new() -> Self;
}

impl Amogus for Amogus1 {
    fn new() -> Self {
        Self
    }
}

impl Amogus for Amogus2 {
    fn new() -> Self {
        Self
    }
}

impl Amogus for Amogus3 {
    fn new() -> Self {
        Self
    }
}

impl Amogus for Amogus4 {
    fn new() -> Self {
        Self
    }
}

impl Amogus for Amogus5 {
    fn new() -> Self {
        Self
    }
}

#[derive(Component)]
struct FlameAnimation(Timer);
#[derive(Component)]
struct SmokeAnimation(Timer);

impl Animation for FlameAnimation {
    fn index() -> usize {
        30
    }

    fn size() -> usize {
        5
    }

    fn timer(&mut self) -> &mut Timer {
        &mut self.0
    }
}

impl Animation for SmokeAnimation {
    fn index() -> usize {
        40
    }

    fn size() -> usize {
        5
    }

    fn timer(&mut self) -> &mut Timer {
        &mut self.0
    }
}

fn click_pickle_mew(
    event: On<Pointer<Press>>,
    mut commands: Commands,
    assets: Res<Assets>,
    mut query: Query<Entity, With<PickleMew>>,
) {
    if let Ok(entity) = query.get_mut(event.event_target()) {
        commands.spawn(AudioPlayer(assets.pickle_mew.clone()));
        commands.entity(entity).despawn();
    }
}
