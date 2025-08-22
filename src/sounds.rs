use bevy::prelude::*;

use crate::{AppState, assets_loader::SceneAssets};

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Failed),
            play_death_explosion_sound,
        )
        .add_systems(
            OnExit(AppState::Failed),
            clean_sound_effects,
        )
        .add_systems(
            OnEnter(AppState::Succeed),
            play_success_sound,
        )
        .add_systems(
            OnExit(AppState::Succeed),
            clean_sound_effects,
        );
    }
}

#[derive(Component)]
struct SoundEffectMarker;

fn play_death_explosion_sound(
    mut commands: Commands,
    sounds: Res<SceneAssets>,
) {
    commands.spawn((
        AudioPlayer::new(sounds.death_explosion_sound.clone()),
        SoundEffectMarker,
    ));
}

fn play_success_sound(
    mut commands: Commands,
    sounds: Res<SceneAssets>,
) {
    commands.spawn((
        AudioPlayer::new(sounds.success_sound.clone()),
        SoundEffectMarker,
    ));
}

// TODO: Think about fade out to prevent sound "click" effect on remove
fn clean_sound_effects(
    mut commands: Commands,
    query: Query<Entity, With<SoundEffectMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
