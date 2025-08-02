mod assets_loader;

use assets_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::assets_loader::SceneAssets;

const IN_DEVELOPMENT: bool = true;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.133, 0.12, 0.12)))
        .insert_resource(AmbientLight {
            brightness: 400.0,
            ..default()
        })
        .add_plugins((DefaultPlugins, AssetLoaderPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, on_update);

    if IN_DEVELOPMENT {
        app.add_systems(Update, exit_on_esc);

        app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()));
    }

    app.run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn(SceneRoot(scene_assets.floor.clone()));
    commands.spawn(SceneRoot(scene_assets.landing_pad.clone()));
    commands.spawn(SceneRoot(scene_assets.launch_pad.clone()));
    commands.spawn((
        SceneRoot(scene_assets.rocket.clone()),
        Transform::from_xyz(-7.5, 1.5, 0.0),
        Player,
    ));

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 65.0f32.to_radians(),
            ..default()
        }),
        Transform::from_xyz(0.0, 4.5, 9.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
    ));
}

fn on_update(
    mut player: Single<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if keys.pressed(KeyCode::Space) {
        player.translation.y -= time.delta_secs();
    }

    if keys.pressed(KeyCode::KeyQ) {
        player.rotate_local_z(time.delta_secs());
    } else if keys.pressed(KeyCode::KeyE) {
        player.rotate_local_z(-time.delta_secs());
    }
}

fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
