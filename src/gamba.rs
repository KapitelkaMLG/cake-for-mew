use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;

use crate::{GameState, assets::Assets, util::despawn_all};

pub struct GambaPlugin;

impl Plugin for GambaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(1))
            .insert_resource(Bet(1))
            .add_message::<BetChangeMessage>()
            .add_message::<BetMessage>()
            .add_systems(OnEnter(GameState::Gamba), setup)
            .add_systems(
                Update,
                (
                    handle_bet_messages,
                    handle_bet_change_messages,
                    despawn_bankruptcy_messages,
                    move_camera,
                )
                    .run_if(in_state(GameState::Gamba)),
            )
            .add_systems(OnExit(GameState::Gamba), despawn_all::<OnGambaScreen>);
    }
}

#[derive(Component)]
struct OnGambaScreen;

const SUGAR_CANE_SCALE: Vec3 = Vec3::splat(2.);
const SUGAR_CANE_SIZE: f32 = 64.;
const SUGAR_CANE_INDEX: usize = 50;
const SUGAR_CANE_VARIANTS: usize = 4;
const SUGAR_CANE_COUNT: usize = 6;
const SIGN_INDEX: usize = 60;

#[derive(Resource)]
pub struct SugarCaneHeight {
    left: [usize; SUGAR_CANE_COUNT],
    right: [usize; SUGAR_CANE_COUNT],
}

#[derive(Component, Clone, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Resource)]
pub struct Score(pub u64);

#[derive(Resource)]
pub struct Bet(pub u64);

#[derive(Message)]
pub struct BetChangeMessage(pub BetChange);

pub enum BetChange {
    Increase,
    Decrease,
}

#[derive(Message)]
pub struct BetMessage(pub Side);

fn setup(
    mut commands: Commands,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    assets: Res<Assets>,
    mut light_query: Query<&mut Light2d, With<Camera>>,
) {
    commands.insert_resource(ClearColor(Color::srgb_u8(63, 127, 255)));
    commands.insert_resource(CameraMoveState {
        start: 0.0,
        target: 0.0,
        timer: Timer::default(),
    });
    for mut light in &mut light_query {
        light.ambient_light.brightness = 1.;
    }

    commands.insert_resource(SugarCaneHeight {
        left: [0, 0, 0, 0, 0, 0],
        right: [0, 0, 0, 0, 0, 0],
    });

    commands.spawn((
        Sprite {
            color: Color::srgb_u8(0, 128, 29),
            ..default()
        },
        OnGambaScreen,
        Transform::from_xyz(0., -3200. - SUGAR_CANE_SIZE / 2., 0.).with_scale(Vec3::splat(6400.)),
    ));

    commands.spawn((
        Sprite::from_atlas_image(
            assets.textures.clone(),
            TextureAtlas::from(assets.texture_atlas.clone()).with_index(SIGN_INDEX),
        ),
        OnGambaScreen,
        Transform::from_scale(SUGAR_CANE_SCALE),
    ));

    (0..SUGAR_CANE_COUNT).for_each(|i| {
        spawn_sugar_cane(
            &mut commands,
            &mut rng,
            assets.textures.clone(),
            TextureAtlas::from(assets.texture_atlas.clone()),
            i,
            0,
            Side::Left,
        );

        spawn_sugar_cane(
            &mut commands,
            &mut rng,
            assets.textures.clone(),
            TextureAtlas::from(assets.texture_atlas.clone()),
            i,
            0,
            Side::Right,
        )
    });
}

fn handle_bet_change_messages(
    mut bet_change_messages: MessageReader<BetChangeMessage>,
    mut bet: ResMut<Bet>,
    score: ResMut<Score>,
) {
    for msg in bet_change_messages.read() {
        match msg.0 {
            BetChange::Increase => {
                if bet.0 < score.0 {
                    bet.0 += 1;
                }
            }
            BetChange::Decrease => {
                if bet.0 > 1 {
                    bet.0 -= 1;
                }
            }
        }
    }
}

fn handle_bet_messages(
    mut commands: Commands,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    assets: Res<Assets>,
    mut bet_messages: MessageReader<BetMessage>,
    mut bet: ResMut<Bet>,
    mut score: ResMut<Score>,
    mut sugar_cane_height: ResMut<SugarCaneHeight>,
    mut camera_move_state: ResMut<CameraMoveState>,
    camera_transform: Single<&Transform, With<Camera>>,
) {
    for msg in bet_messages.read() {
        let choice = if rng.next_u32().is_multiple_of(2) {
            Side::Left
        } else {
            Side::Right
        };
        if msg.0 == choice {
            score.0 += bet.0;
        } else {
            score.0 -= bet.0;
            if score.0 == 0 {
                declare_bankruptcy(&mut commands, &mut rng, &mut score);
            }
            bet.0 = bet.0.min(score.0);
        }
        let index = rng.next_u32() as usize % SUGAR_CANE_COUNT;
        camera_move_state.start = camera_transform.translation.y;
        camera_move_state.timer = Timer::from_seconds(2., TimerMode::Once);
        match choice {
            Side::Left => {
                sugar_cane_height.left[index] += 1;
                camera_move_state.target = sugar_cane_height.left[index] as f32 * SUGAR_CANE_SIZE;
                spawn_sugar_cane(
                    &mut commands,
                    &mut rng,
                    assets.textures.clone(),
                    TextureAtlas::from(assets.texture_atlas.clone()),
                    index,
                    sugar_cane_height.left[index],
                    choice,
                );
            }
            Side::Right => {
                sugar_cane_height.right[index] += 1;
                camera_move_state.target = sugar_cane_height.right[index] as f32 * SUGAR_CANE_SIZE;
                spawn_sugar_cane(
                    &mut commands,
                    &mut rng,
                    assets.textures.clone(),
                    TextureAtlas::from(assets.texture_atlas.clone()),
                    index,
                    sugar_cane_height.right[index],
                    choice,
                );
            }
        }
    }
}

fn spawn_sugar_cane(
    commands: &mut Commands,
    rng: &mut WyRand,
    textures: Handle<Image>,
    atlas: TextureAtlas,
    index: usize,
    height: usize,
    side: Side,
) {
    let side_sign = match side {
        Side::Left => -1.,
        Side::Right => 1.,
    };
    let variant = rng.next_u32() as usize % SUGAR_CANE_VARIANTS;
    commands
        .spawn((
            Sprite::from_atlas_image(textures, atlas.with_index(SUGAR_CANE_INDEX + variant)),
            OnGambaScreen,
            side,
            Pickable::default(),
            Transform::from_translation(Vec3::new(
                side_sign * (index + 1) as f32 * SUGAR_CANE_SIZE,
                height as f32 * SUGAR_CANE_SIZE,
                0.,
            ))
            .with_scale(SUGAR_CANE_SCALE),
        ))
        .observe(pick_side);
}

fn pick_side(
    event: On<Pointer<Press>>,
    mut message_writer: MessageWriter<BetMessage>,
    side_query: Query<&Side>,
) {
    if let Ok(side) = side_query.get(event.event_target()) {
        message_writer.write(BetMessage(side.clone()));
    }
}

#[derive(Component)]
struct BrokeMessage(Timer);

fn declare_bankruptcy(commands: &mut Commands, rng: &mut WyRand, score: &mut Score) {
    score.0 = rng.next_u32() as u64 % 5 + 5;

    let id = commands.register_system(despawn_all::<BrokeMessage>);
    commands.run_system(id);

    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(20.),
            height: Val::Percent(20.),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(30.),
            left: Val::Percent(30.),
            ..default()
        },
        BrokeMessage(Timer::from_seconds(2.5, TimerMode::Once)),
        OnGambaScreen,
        children![(
            Text::new(format!("Damn, you're broke! Here, have ${}", score.0)),
            TextFont {
                font_size: 24.,
                ..default()
            },
            TextColor(Color::srgb_u8(29, 29, 29)),
        )],
    ));
}

fn despawn_bankruptcy_messages(
    mut commands: Commands,
    mut broke_messages: Query<(Entity, &mut BrokeMessage)>,
    time: Res<Time>,
) {
    for (entity, mut msg) in &mut broke_messages {
        let timer = &mut msg.0;
        timer.tick(time.delta());
        if timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Resource)]
struct CameraMoveState {
    start: f32,
    target: f32,
    timer: Timer,
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut state: ResMut<CameraMoveState>,
    time: Res<Time>,
) {
    if !state.timer.is_finished() {
        state.timer.tick(time.delta());
        for mut transform in &mut camera_query {
            transform.translation.y = state.start.lerp(state.target, state.timer.fraction());
        }
    }
}
