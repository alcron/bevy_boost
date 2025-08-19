use avian3d::prelude::*;
use bevy::prelude::*;

pub use avian3d::prelude::{
    AngularVelocity, LinearVelocity, RigidBody,
};

use crate::IN_DEVELOPMENT;

pub const IS_COLLIDER_WIREFRAME_ENABLED: bool = false;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugins::default(),))
            .add_event::<CollisionEvent>()
            .add_systems(Update, on_collide);

        if IN_DEVELOPMENT && IS_COLLIDER_WIREFRAME_ENABLED {
            app.add_plugins(PhysicsDebugPlugin::default());
        }
    }
}

pub fn create_collider(
    collider_type: RigidBody,
    collider: Handle<Mesh>,
) -> impl Bundle {
    (
        collider_type,
        Mesh3d(collider.clone()),
        Mass(100.0),
        ColliderDensity::default(),
        Restitution::new(0.01),
        Friction::new(0.8),
        CollisionMargin(0.05),
        ColliderConstructor::TrimeshFromMesh,
        CollisionEventsEnabled,
    )
}

#[derive(Event)]
pub struct CollisionEvent(pub Entity, pub Entity);

fn on_collide(
    mut started: EventReader<CollisionStarted>,
    mut ew_collision: EventWriter<CollisionEvent>,
) {
    for ev in started.read() {
        ew_collision.write(CollisionEvent(ev.0, ev.1));
    }
}
