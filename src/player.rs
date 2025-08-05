use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{AppState, assets_loader::SceneAssets};

#[derive(Component)]
pub struct Player;

#[derive(
    Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect,
)]
enum Action {
    Boost,
    RotateLeft,
    RotateRight,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<Action>::default(),
        ))
        // TODO: Make proper schedule for spawning player after level
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(
            Update,
            on_update.run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::GameOver), despawn);
    }
}

fn setup(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
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

// TODO: Refactor level function duplicate
fn despawn(
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
) {
    commands.entity(player.entity()).despawn();
}
