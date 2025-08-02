use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub floor: Handle<Scene>,
    pub landing_pad: Handle<Scene>,
    pub launch_pad: Handle<Scene>,
    pub rocket: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        floor: asset_server.load(GltfAssetLabel::Scene(0).from_asset("floor.glb")),
        landing_pad: asset_server.load(GltfAssetLabel::Scene(0).from_asset("landing_pad.glb")),
        launch_pad: asset_server.load(GltfAssetLabel::Scene(0).from_asset("launch_pad.glb")),
        rocket: asset_server.load(GltfAssetLabel::Scene(0).from_asset("rocket.glb")),
    };
}
