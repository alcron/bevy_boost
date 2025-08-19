mod tween_move;

use bevy::prelude::*;

use crate::{
    assets_loader::{GameAsset, SceneAssets},
    collision::{RigidBody, create_collider},
    level::tween_move::TweenMove,
};

use tween_move::TweenMovePlugin;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeEvent>()
            .add_plugins(TweenMovePlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, on_change);
    }
}

fn setup(mut ew_change: EventWriter<ChangeEvent>) {
    ew_change.write(ChangeEvent::Set(Level::First));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    First,
    Second,
    Third,
    Fourth,
}

const LEVELS_ORDER: [Level; 4] =
    [Level::First, Level::Second, Level::Third, Level::Fourth];

#[derive(Component)]
struct LevelMarker;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Finish;

#[derive(Event, Debug)]
pub enum ChangeEvent {
    Next,
    Set(Level),
    Reload,
}

fn create_static_asset(game_asset: GameAsset) -> impl Bundle {
    let collider = game_asset.collider.clone().unwrap();

    (
        SceneRoot(game_asset.model.clone()),
        create_collider(RigidBody::Static, collider),
    )
}

fn on_change(
    mut commands: Commands,
    mut er_change: EventReader<ChangeEvent>,
    mut current_level_index: Local<usize>,
    scene_assets: Res<SceneAssets>,
    level_query: Query<Entity, With<LevelMarker>>,
) {
    for ev in er_change.read() {
        for entity in level_query.iter() {
            commands.entity(entity).despawn();
        }
        let level_to_load_index = match ev {
            ChangeEvent::Next => {
                (*current_level_index + 1) % LEVELS_ORDER.len()
            }
            ChangeEvent::Set(level) => LEVELS_ORDER
                .iter()
                .position(|l| l == level)
                .unwrap_or(0),
            ChangeEvent::Reload => *current_level_index,
        };

        *current_level_index = level_to_load_index;

        commands
            .spawn((
                Transform::from_xyz(0.0, 0.0, 0.0),
                Visibility::default(),
                LevelMarker,
            ))
            .with_children(|level| {
                level.spawn((
                    create_static_asset(
                        scene_assets.floor.clone(),
                    ),
                    Name::new("Floor"),
                    Obstacle,
                ));

                level.spawn((
                    create_static_asset(
                        scene_assets.landing_pad.clone(),
                    ),
                    Name::new("LandingPad"),
                    Finish,
                ));

                level.spawn((
                    create_static_asset(
                        scene_assets.launch_pad.clone(),
                    ),
                    Name::new("LaunchPad"),
                ));

                match LEVELS_ORDER[level_to_load_index] {
                    Level::Second => {
                        level.spawn((
                            create_static_asset(
                                scene_assets.obstacle_2.clone(),
                            ),
                            Name::new("2_Obstacle"),
                            Obstacle,
                        ));
                    }
                    Level::Third => {
                        level.spawn((
                            create_static_asset(
                                scene_assets.obstacle_3.clone(),
                            ),
                            Name::new("3_Obstacle"),
                            Obstacle,
                        ));
                    }
                    Level::Fourth => {
                        level.spawn((
                            // TODO: Make own asset
                            SceneRoot(
                                scene_assets
                                    .obstacle_2
                                    .model
                                    .clone(),
                            ),
                            create_collider(
                                RigidBody::Static,
                                scene_assets
                                    .obstacle_2
                                    .collider
                                    .clone()
                                    .unwrap(),
                            ),
                            Name::new("4_Obstacle"),
                            Obstacle,
                            TweenMove {
                                target: Vec3::new(0.0, 2.5, 0.0),
                                duration: 2,
                            },
                        ));
                    }
                    _ => {}
                }
            });
    }
}
