use bevy::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct GameAsset {
    pub model: Handle<Scene>,
    pub collider: Option<Handle<Mesh>>,
}

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub floor: GameAsset,
    pub launch_pad: GameAsset,
    pub landing_pad: GameAsset,
    pub rocket: GameAsset,
    pub obstacle_2: GameAsset,
    pub obstacle_3: GameAsset,
    pub death_explosion_sound: Handle<AudioSource>,
    pub success_sound: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
) {
    // TODO: Make proper assets handling and load using one gltf file (check https://bevy.org/examples/assets/multi-asset-sync/ for example)
    // TODO: Think about better implementation for loading meshes

    *scene_assets = SceneAssets {
        floor: GameAsset {
            model: asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("floor.gltf"),
            ),
            collider: Some(
                asset_server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset("floor.gltf"),
                ),
            ),
        },
        launch_pad: GameAsset {
            model: asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("launch_pad.gltf"),
            ),
            collider: Some(
                asset_server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset("launch_pad.gltf"),
                ),
            ),
        },
        landing_pad: GameAsset {
            model: asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("landing_pad.gltf"),
            ),
            collider: Some(
                asset_server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset("landing_pad.gltf"),
                ),
            ),
        },
        rocket: GameAsset {
            model: asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("rocket.gltf"),
            ),
            collider: Some(
                asset_server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset("rocket_collider.gltf"),
                ),
            ),
        },
        obstacle_2: GameAsset {
            model: asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("2_level/obstacle.gltf"),
            ),
            collider: Some(
                asset_server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset("2_level/obstacle.gltf"),
                ),
            ),
        },
        obstacle_3: GameAsset {
            model: asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("3_level/obstacle.gltf"),
            ),
            collider: Some(
                asset_server.load(
                    GltfAssetLabel::Primitive {
                        mesh: 0,
                        primitive: 0,
                    }
                    .from_asset("3_level/obstacle.gltf"),
                ),
            ),
        },
        death_explosion_sound: asset_server
            .load("sounds/death_explosion.ogg"),
        success_sound: asset_server.load("sounds/success.ogg"),
    };
}
