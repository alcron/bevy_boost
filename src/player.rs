use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    AppState,
    assets_loader::SceneAssets,
    collision::{
        AngularVelocity, ColliderType, CollisionEvent,
        LinearVelocity, create_collider,
    },
    level::{Finish, Obstacle},
};

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

#[derive(Resource, Default)]
pub struct TriesCounter(pub u32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TriesCounter>()
            .add_plugins((
                InputManagerPlugin::<Action>::default(),
            ))
            .add_systems(
                OnEnter(AppState::InGame),
                increment_tries_counter,
            )
            .add_systems(
                Update,
                (on_update, on_obstacle_or_finish_collision)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnEnter(AppState::InGame),
                (despawn, setup).chain(),
            );
    }
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

// TODO: Split logics into several systems
fn on_obstacle_or_finish_collision(
    mut er_collision: EventReader<CollisionEvent>,
    tries_counter: Res<TriesCounter>,
    mut was_triggered_on_this_run: Local<bool>,
    mut current_try: Local<u32>,
    mut next_state: ResMut<NextState<AppState>>,
    player_query: Query<(), With<Player>>,
    obstacle_query: Query<(), With<Obstacle>>,
    finish_query: Query<(), With<Finish>>,
) {
    if *current_try != tries_counter.0 {
        *was_triggered_on_this_run = false;
        *current_try = tries_counter.0;
    }

    for ev in er_collision.read() {
        if *was_triggered_on_this_run {
            continue;
        }

        let is_player_and_obstacle_collided =
            player_query.get(ev.0).is_ok()
                && obstacle_query.get(ev.1).is_ok()
                || player_query.get(ev.1).is_ok()
                    && obstacle_query.get(ev.0).is_ok();
        let is_player_and_finish_collided =
            player_query.get(ev.0).is_ok()
                && finish_query.get(ev.1).is_ok()
                || player_query.get(ev.1).is_ok()
                    && finish_query.get(ev.0).is_ok();

        if is_player_and_obstacle_collided {
            next_state.set(AppState::Failed);
        }
        if is_player_and_finish_collided {
            next_state.set(AppState::Succeed);
        }

        if is_player_and_obstacle_collided
            || is_player_and_finish_collided
        {
            *was_triggered_on_this_run = true;
        }
    }
}

fn setup(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    let collider = scene_assets.rocket.collider.clone().unwrap();

    commands.spawn((
        SceneRoot(scene_assets.rocket.model.clone()),
        Transform::from_xyz(-7.5, 2.5, 0.0),
        create_collider(ColliderType::Dynamic, collider),
        InputMap::new([
            (Action::Boost, KeyCode::Space),
            (Action::Boost, KeyCode::KeyW),
            (Action::RotateLeft, KeyCode::KeyA),
            (Action::RotateLeft, KeyCode::ArrowLeft),
            (Action::RotateRight, KeyCode::KeyD),
            (Action::RotateRight, KeyCode::ArrowRight),
        ]),
        Name::new("Player"),
        Player,
    ));
}

fn despawn(
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
) {
    commands.entity(*player).despawn();
}

fn increment_tries_counter(
    mut tries_counter: ResMut<TriesCounter>,
) {
    tries_counter.0 += 1;
}
