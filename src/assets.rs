use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_gltf_blueprints::*;


#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum AssetState {
    #[default]
    Loading,
    Ready,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "blueprints/scene.glb")]
    pub scene: Handle<bevy::gltf::Gltf>,
    #[asset(path = "blueprints/rotunda.glb")]
    pub rotunda: Handle<bevy::gltf::Gltf>,
    #[asset(path = "blueprints/island.glb")]
    pub island: Handle<bevy::gltf::Gltf>,
    #[asset(path = "blueprints/water.glb")]
    pub water: Handle<bevy::gltf::Gltf>,
}


pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        // Loading state setup
        app.init_state::<AssetState>();
        app.add_loading_state(
            LoadingState::new(AssetState::Loading)
                .load_collection::<GameAssets>()
                .continue_to_state(AssetState::Ready),
        );
        // Blueprints setup
        app.add_plugins(BlueprintsPlugin {
            legacy_mode: false,
            library_folder: "blueprints/".into(),
            ..default()
        });
        // Spawning scene
        app.add_systems(OnEnter(AssetState::Ready), spawn_scene);
    }
}


fn spawn_scene(
    mut commands: Commands,
) {
    commands.spawn((
        BlueprintName("scene".to_string()),
        SpawnHere,
        SpatialBundle::default(),
    ));
}
