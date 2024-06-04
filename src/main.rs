use bevy::prelude::*;

mod types;
mod assets;
mod controls;
mod lighting;
mod player;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            assets::AssetsPlugin,
            controls::ControlPlugin,
            lighting::LightingPlugin,
            player::PlayerPlugin,
        ))
        .run();
}


