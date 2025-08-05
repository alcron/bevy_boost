use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    AppState, assets_loader::SceneAssets, player::Player,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(OnExit(AppState::GameOver), despawn);
    }
}

#[derive(Component)]
struct Level;

fn setup(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
            Level,
        ))
        .with_children(|level| {
            level
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
                     player_query: Query<&Player>,
                     mut next_state: ResMut<
                        NextState<AppState>,
                    >| {
                        if player_query
                            .contains(trigger.collider)
                        {
                            next_state.set(AppState::GameOver);
                        }
                    },
                );

            level
                .spawn((
                    SceneRoot(scene_assets.landing_pad.clone()),
                    RigidBody::Static,
                    Restitution::new(0.1),
                    Mesh3d(
                        scene_assets.landing_pad_mesh.clone(),
                    ),
                    ColliderConstructor::ConvexHullFromMesh,
                    CollisionEventsEnabled,
                ))
                .observe(
                    |trigger: Trigger<OnCollisionStart>,
                     player_query: Query<&Player>,
                     mut next_state: ResMut<
                        NextState<AppState>,
                    >| {
                        if player_query
                            .contains(trigger.collider)
                        {
                            next_state.set(AppState::GameOver);
                        }
                    },
                );

            level.spawn((
                SceneRoot(scene_assets.launch_pad.clone()),
                RigidBody::Static,
                Restitution::new(0.1),
                Mesh3d(scene_assets.launch_pad_mesh.clone()),
                ColliderConstructor::ConvexHullFromMesh,
            ));
        });
}

fn despawn(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    level: Single<Entity, With<Level>>,
) {
    if keys.pressed(KeyCode::KeyQ) {
        commands.entity(level.entity()).despawn();
    }
}
