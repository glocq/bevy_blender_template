use bevy::prelude::*;


pub struct LightingPlugin;

impl Plugin for LightingPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_lights);
    }
}

// Create an ambient light and a directional light from 45 degrees above the horizon.
// Also, set the background color to sky blue.
fn init_lights(
    mut commands: Commands,
) {
    // No idea why this color appears more vivid in-game than in a color picker?
    commands.insert_resource(ClearColor(Color::rgb(0.6, 0.75, 0.9))); // blue

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            // Default light points towards negative z, i.e. from above,
            // so we rotate it by 45 degrees and end up 45 degrees above the horizon.
            rotation: Quat::from_rotation_x(std::f32::consts::PI / 4.0),
            ..default()
        },
        ..default()
    });
}
