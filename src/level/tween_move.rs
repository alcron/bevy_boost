use bevy::prelude::*;
use bevy_tween::interpolate;
use bevy_tween::prelude::*;

#[derive(Component)]
#[require(Transform)]
pub struct TweenMove {
    pub target: Vec3,
    pub duration: usize,
}

pub struct TweenMovePlugin;

impl Plugin for TweenMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultTweenPlugins)
            .add_systems(Update, handle_move);
    }
}

fn handle_move(
    mut commands: Commands,
    mut query: Query<(Entity, &TweenMove), Added<TweenMove>>,
) {
    for (entity, move_config) in query.iter_mut() {
        let target = entity.into_target();
        let mut transform_state = target.state(Vec3::ZERO);

        commands
            .entity(entity)
            .animation()
            .repeat(Repeat::Infinitely)
            .repeat_style(RepeatStyle::PingPong)
            .insert_tween_here(
                Duration::from_secs(move_config.duration as u64),
                EaseKind::SineInOut,
                transform_state.with(
                    interpolate::translation_to(
                        move_config.target,
                    ),
                ),
            );
    }
}
