use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub floor: Handle<Scene>,
    pub floor_mesh: Handle<Mesh>,
    pub landing_pad: Handle<Scene>,
    pub landing_pad_mesh: Handle<Mesh>,
    pub launch_pad: Handle<Scene>,
    pub launch_pad_mesh: Handle<Mesh>,
    pub rocket: Handle<Scene>,
    pub rocket_mesh: Handle<Mesh>,
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
    // For proper positioning mesh shoud apply transformation in Blender (move object origin to center of world)
    let floor_mesh: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("floor.glb"),
    );
    let launch_pad_mesh: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("launch_pad.glb"),
    );
    let landing_pad_mesh: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("landing_pad.glb"),
    );
    let rocket_mesh: Handle<Mesh> = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("rocket.glb"),
    );

    // TODO: Make proper assets handling and load using one gltf file (check https://bevy.org/examples/assets/multi-asset-sync/ for example)
    // TODO: Think about better implementation for loading meshes

    *scene_assets = SceneAssets {
        floor: asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("floor.glb"),
        ),
        floor_mesh,
        landing_pad: asset_server.load(
            GltfAssetLabel::Scene(0)
                .from_asset("landing_pad.glb"),
        ),
        landing_pad_mesh,
        launch_pad: asset_server.load(
            GltfAssetLabel::Scene(0)
                .from_asset("launch_pad.glb"),
        ),
        launch_pad_mesh,
        rocket: asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("rocket.glb"),
        ),
        rocket_mesh,
    };
}
