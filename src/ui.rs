use bevy::prelude::*;

use crate::{
    GameState,
    gamba::{Bet, BetChange, BetChangeMessage, BetMessage, Score, Side},
    util::despawn_all,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AssetLoading), setup_asset_loading)
            .add_systems(
                OnExit(GameState::AssetLoading),
                despawn_all::<OnAssetLoadingScreen>,
            )
            .add_systems(OnEnter(GameState::Cake), setup_cake)
            .add_systems(OnExit(GameState::Cake), despawn_all::<OnCakeScreen>)
            .add_systems(OnEnter(GameState::Gamba), setup_gamba)
            .add_systems(Update, update_displays.run_if(in_state(GameState::Gamba)))
            .add_systems(OnExit(GameState::Gamba), despawn_all::<OnGambaScreen>)
            .add_systems(Update, handle_buttons);
    }
}

#[derive(Component)]
struct OnAssetLoadingScreen;

fn setup_asset_loading(mut commands: Commands) {
    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        OnAssetLoadingScreen,
        children![(
            Text::new("Loading assets..."),
            TextFont {
                font_size: 69.,
                ..default()
            },
        )],
    ));
}

#[derive(Component)]
struct OnCakeScreen;

fn setup_cake(mut commands: Commands) {
    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            ..default()
        },
        OnCakeScreen,
        children![(
            Text::new("Happy birthday, Mew <3 <3 <3"),
            TextFont {
                font_size: 69.,
                ..default()
            },
            TextColor(Color::srgb_u8(243, 207, 198)),
        )],
    ));

    commands.spawn((
        Button,
        ButtonAction::Gamba,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(120.),
            height: Val::Px(60.),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(0.),
            right: Val::Percent(0.),
            ..default()
        },
        OnCakeScreen,
        children![(
            Text::new("secret gamba"),
            TextFont {
                font_size: 12.,
                ..default()
            },
            TextColor(Color::srgb_u8(69, 69, 69)),
        )],
    ));
}

#[derive(Component)]
struct OnGambaScreen;

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct BetDisplay;

fn setup_gamba(mut commands: Commands) {
    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            ..default()
        },
        OnGambaScreen,
        children![(
            Text::new("$"),
            TextFont {
                font_size: 24.,
                ..default()
            },
            children![(TextSpan::default(), ScoreDisplay,)],
        )],
    ));

    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(20.),
            height: Val::Percent(20.),
            position_type: PositionType::Absolute,
            right: Val::Percent(20.),
            bottom: Val::Percent(20.),
            ..default()
        },
        OnGambaScreen,
        children![
            (
                Button,
                ButtonAction::DecreaseBet,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(48.),
                    height: Val::Px(48.),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(128, 0, 0)),
                children![(
                    Text::new("<"),
                    TextFont {
                        font_size: 24.,
                        ..default()
                    },
                )],
            ),
            (
                Text::new("bet: $"),
                TextFont {
                    font_size: 24.,
                    ..default()
                },
                children![(TextSpan::default(), BetDisplay,)],
            ),
            (
                Button,
                ButtonAction::IncreaseBet,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(48.),
                    height: Val::Px(48.),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(0, 128, 0)),
                children![(
                    Text::new(">"),
                    TextFont {
                        font_size: 24.,
                        ..default()
                    },
                )],
            )
        ],
    ));

    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Percent(40.),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(0.),
            ..default()
        },
        OnGambaScreen,
        children![
            (
                Button,
                ButtonAction::BetLeft,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(96.),
                    height: Val::Px(48.),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(128, 128, 128)),
                children![(
                    Text::new("LEFT"),
                    TextFont {
                        font_size: 24.,
                        ..default()
                    },
                )],
            ),
            (
                Text::new("<->"),
                TextFont {
                    font_size: 24.,
                    ..default()
                },
            ),
            (
                Button,
                ButtonAction::BetRight,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(96.),
                    height: Val::Px(48.),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(128, 128, 128)),
                children![(
                    Text::new("RIGHT"),
                    TextFont {
                        font_size: 24.,
                        ..default()
                    },
                )],
            )
        ],
    ));

    commands.spawn((
        Button,
        ButtonAction::Cake,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(240.),
            height: Val::Px(60.),
            position_type: PositionType::Absolute,
            bottom: Val::Percent(0.),
            right: Val::Percent(0.),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(200, 200, 200)),
        OnGambaScreen,
        children![(
            Text::new("Want more cake?"),
            TextFont {
                font_size: 24.,
                ..default()
            },
            TextColor(Color::srgb_u8(29, 29, 29)),
        )],
    ));
}

fn update_displays(
    mut score_query: Query<&mut TextSpan, (With<ScoreDisplay>, Without<BetDisplay>)>,
    score: Res<Score>,
    mut bet_query: Query<&mut TextSpan, (With<BetDisplay>, Without<ScoreDisplay>)>,
    bet: Res<Bet>,
) {
    for mut span in &mut score_query {
        **span = match score.0 {
            69 => format!("{}, nice", score.0),
            _ => format!("{}", score.0),
        };
    }
    for mut span in &mut bet_query {
        **span = format!("{}", bet.0);
    }
}

#[derive(Component)]
enum ButtonAction {
    Cake,
    Gamba,
    IncreaseBet,
    DecreaseBet,
    BetLeft,
    BetRight,
}

fn handle_buttons(
    interaction_query: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut bet_change_message_writer: MessageWriter<BetChangeMessage>,
    mut bet_message_writer: MessageWriter<BetMessage>,
) {
    for (interaction, button_action) in interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::Cake => {
                    game_state.set(GameState::Cake);
                }
                ButtonAction::Gamba => {
                    game_state.set(GameState::Gamba);
                }
                ButtonAction::IncreaseBet => {
                    bet_change_message_writer.write(BetChangeMessage(BetChange::Increase));
                }
                ButtonAction::DecreaseBet => {
                    bet_change_message_writer.write(BetChangeMessage(BetChange::Decrease));
                }
                ButtonAction::BetLeft => {
                    bet_message_writer.write(BetMessage(Side::Left));
                }
                ButtonAction::BetRight => {
                    bet_message_writer.write(BetMessage(Side::Right));
                }
            }
        }
    }
}
