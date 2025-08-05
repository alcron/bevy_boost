mod assets_loader;
mod level;
mod player;

use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin, quick::WorldInspectorPlugin,
};

use crate::{level::LevelPlugin, player::PlayerPlugin};
use assets_loader::AssetLoaderPlugin;

const IN_DEVELOPMENT: bool = true;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Setup,
    InGame,
    GameOver,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(
            0.133, 0.12, 0.12,
        )))
        .init_state::<AppState>()
        .insert_resource(AmbientLight {
            brightness: 400.0,
            ..default()
        })
        .add_plugins((
            AssetLoaderPlugin,
            LevelPlugin,
            PlayerPlugin,
        ))
        .add_systems(OnEnter(AppState::Setup), setup)
        .add_systems(
            Update,
            start_game.run_if(in_state(AppState::Setup)),
        )
        .add_systems(
            Update,
            on_restart.run_if(in_state(AppState::GameOver)),
        );

    if IN_DEVELOPMENT {
        app.add_systems(Update, exit_on_esc);

        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::default(),
        ));
    }

    app.run();
}

fn start_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Main camera"),
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 65.0f32.to_radians(),
            ..default()
        }),
        Transform::from_xyz(0.0, 4.5, 9.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
    ));
}

fn on_restart(
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        next_state.set(AppState::InGame);
    }
}

fn exit_on_esc(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
