mod assets_loader;

use assets_loader::AssetLoaderPlugin;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin, quick::WorldInspectorPlugin,
};
use leafwing_input_manager::prelude::*;

use crate::assets_loader::SceneAssets;

const IN_DEVELOPMENT: bool = true;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(
        0.133, 0.12, 0.12,
    )))
    .insert_resource(AmbientLight {
        brightness: 400.0,
        ..default()
    })
    .add_plugins((
        DefaultPlugins,
        AssetLoaderPlugin,
        InputManagerPlugin::<Action>::default(),
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, on_update);

    if IN_DEVELOPMENT {
        app.add_systems(Update, exit_on_esc);

        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::default(),
        ));
    }

    app.run();
}

#[derive(Component)]
struct Player;

#[derive(
    Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect,
)]
enum Action {
    Boost,
    RotateLeft,
    RotateRight,
}

fn setup(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands
        .spawn((
            SceneRoot(scene_assets.floor.clone()),
            RigidBody::Static,
            Restitution::new(0.1),
            Mesh3d(scene_assets.floor_mesh.clone()),
            CollisionEventsEnabled,
            ColliderConstructor::TrimeshFromMesh,
        ))
        .observe(
            |trigger: Trigger<OnCollisionStart>,
             player_query: Query<&Player>| {
                if player_query.contains(trigger.collider) {
                    println!("You lost...");
                }
            },
        );

    commands
        .spawn((
            SceneRoot(scene_assets.landing_pad.clone()),
            RigidBody::Static,
            Restitution::new(0.1),
            Mesh3d(scene_assets.landing_pad_mesh.clone()),
            ColliderConstructor::ConvexHullFromMesh,
            CollisionEventsEnabled,
        ))
        .observe(
            |trigger: Trigger<OnCollisionStart>,
             player_query: Query<&Player>| {
                if player_query.contains(trigger.collider) {
                    println!("You win!");
                }
            },
        );

    commands.spawn((
        SceneRoot(scene_assets.launch_pad.clone()),
        RigidBody::Static,
        Restitution::new(0.1),
        Mesh3d(scene_assets.launch_pad_mesh.clone()),
        ColliderConstructor::ConvexHullFromMesh,
    ));

    commands.spawn((
        SceneRoot(scene_assets.rocket.clone()),
        Transform::from_xyz(-7.5, 2.5, 0.0),
        // TODO: Split to separate plugins
        // Physics components --------------------
        RigidBody::Dynamic,
        Restitution::new(0.1),
        Mesh3d(scene_assets.rocket_mesh.clone()),
        ColliderConstructor::ConvexHullFromMesh,
        LockedAxes::new().lock_translation_z().lock_rotation_x(),
        CollisionEventsEnabled,
        // ---------------------------------------
        InputMap::new([
            (Action::Boost, KeyCode::Space),
            (Action::Boost, KeyCode::KeyW),
            (Action::RotateLeft, KeyCode::KeyA),
            (Action::RotateLeft, KeyCode::ArrowLeft),
            (Action::RotateRight, KeyCode::KeyD),
            (Action::RotateRight, KeyCode::ArrowRight),
        ]),
        Player,
    ));

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 65.0f32.to_radians(),
            ..default()
        }),
        Transform::from_xyz(0.0, 4.5, 9.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
    ));
}

fn on_update(
    mut player: Single<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &ActionState<Action>,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (
        transform,
        linear_velocity,
        angular_velocity,
        action_state,
    ) = &mut *player;

    if action_state.pressed(&Action::Boost) {
        let top = transform.rotation * Vec3::Y;
        let multiplier = time.delta_secs() * 15.0;

        linear_velocity.x += top.x * multiplier;
        linear_velocity.y += top.y * multiplier;
    }

    if action_state.pressed(&Action::RotateLeft) {
        angular_velocity.z += time.delta_secs() * 5.0;
    } else if action_state.pressed(&Action::RotateRight) {
        angular_velocity.z -= time.delta_secs() * 5.0;
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
