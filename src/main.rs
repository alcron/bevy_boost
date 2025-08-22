mod assets_loader;
mod collision;
mod level;
mod player;
mod sounds;

use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin, quick::WorldInspectorPlugin,
};
use iyes_perf_ui::prelude::*;

use crate::{
    collision::CollisionPlugin, level::LevelPlugin,
    player::PlayerPlugin, sounds::SoundsPlugin,
};
use assets_loader::AssetLoaderPlugin;

pub const IN_DEVELOPMENT: bool = true;
pub const IS_PERFOMANCE_PANEL_ENABLED: bool = false;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Setup,
    InGame,
    Failed,
    Succeed,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // Turn off vsync to maximize CPU/GPU usage
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::srgb(0.133, 0.12, 0.12)))
    .init_state::<AppState>()
    .insert_resource(AmbientLight {
        brightness: 400.0,
        ..default()
    })
    .add_plugins((
        AssetLoaderPlugin,
        CollisionPlugin,
        LevelPlugin,
        PlayerPlugin,
        SoundsPlugin,
    ))
    .add_systems(OnEnter(AppState::Setup), setup)
    .add_systems(
        Update,
        start_game.run_if(in_state(AppState::Setup)),
    )
    .add_systems(
        Update,
        on_results.run_if(
            in_state(AppState::Failed)
                .or(in_state(AppState::Succeed)),
        ),
    );

    if IN_DEVELOPMENT {
        app.add_systems(Update, exit_on_esc);

        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::default(),
        ));

        if IS_PERFOMANCE_PANEL_ENABLED {
            app
                .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
                .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
                .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
                .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
                .add_plugins(PerfUiPlugin);
        }
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

    if IN_DEVELOPMENT {
        commands.spawn(PerfUiAllEntries::default());
    }
}

fn on_results(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ew_change: EventWriter<level::ChangeEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if state.get() == &AppState::Failed {
            ew_change.write(level::ChangeEvent::Reload);
            next_state.set(AppState::InGame);
        } else if state.get() == &AppState::Succeed {
            ew_change.write(level::ChangeEvent::Next);
            next_state.set(AppState::InGame);
        }
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
