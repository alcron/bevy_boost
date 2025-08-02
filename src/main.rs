use bevy::prelude::*;

const IN_DEVELOPMENT: bool = true;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, on_update);

    if IN_DEVELOPMENT {
        app.add_systems(Update, exit_on_esc);
    }

    app.run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
        player.rotate_z(time.delta_secs());
    } else if keys.pressed(KeyCode::KeyE) {
        player.rotate_z(-time.delta_secs());
    }
}

fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
