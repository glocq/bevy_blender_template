use bevy::prelude::*;
use crate::types::InputEvent;



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (process_input, update_camera).chain());
    }
}


#[derive(Component)]
struct Player;

/// This is equivalent to Direction3d, but easier to deal with in some cases.
/// Angles are as specified inhttps://en.wikipedia.org/wiki/Horizontal_coordinate_system,
/// except they are in radian instead of degrees.
/// Azimuth: North = 0, East = Pi/4, South = Pi, West = 3*Pi/4
/// Altitude: Horizontal = 0, Up = Pi/4, Down = -Pi/4
#[derive(Component, Default)]
struct HorizontalCoordinates {
    azimuth:  f32,
    altitude: f32,
}



fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player entity: a cube (for now) with a Player marker
    commands.spawn((
        Player,
        PbrBundle {
            mesh:     meshes.add(Cuboid::default()),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 3.2),
                ..default()
            },
            ..default()
        },
    ));

    // A camera, together with an orientation specified as a set of horizontal
    // coordinates. Those are redundant with the camera's `Transform`,
    // but easier to deal with in the context of a game with constant gravity.
    // So what we do is update the horizontal coordinates based on input,
    // and then update the camera's `Transform` accordingly.
    commands.spawn((
        Camera3dBundle::default(),
        HorizontalCoordinates::default(),
    ));
}


fn process_input(
    mut event_input: EventReader<InputEvent>,
    mut player_query: Query<&mut Transform, With<Player>>,
    // `Without<Player>` is conceptually useless here, but it is needed
    // for Bevy not to freak out because of the simultaneous component access
    mut camera_query: Query<&mut HorizontalCoordinates, (With<Camera>, Without<Player>)>,
) {
    let mut player_transform = player_query.single_mut();
    let mut camera_horiz_coord = camera_query.single_mut();

    for event in event_input.read() {
        match event {
            InputEvent::MoveForward(amount) => {
                player_transform.translation += *amount * Quat::from_rotation_z(-camera_horiz_coord.azimuth).mul_vec3(Vec3::X);
            }
            InputEvent::MoveLeft(amount) => {
                player_transform.translation += *amount * Quat::from_rotation_z(-camera_horiz_coord.azimuth).mul_vec3(Vec3::Y);
            }
            InputEvent::MoveUp(amount) => {
                player_transform.translation += *amount * Vec3::Z;
            }
            InputEvent::TurnLeft(amount) => {
                camera_horiz_coord.azimuth -= *amount;
            }
            InputEvent::TurnUp(amount) => {
                camera_horiz_coord.altitude += *amount;
            }
        }
    }
}

fn update_camera(
    mut camera_query: Query<(&mut Transform, &HorizontalCoordinates), With<Camera>>,
    // `Without<Player>` is conceptually useless here, but it is needed
    // for Bevy not to freak out because of the simultaneous component access
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let (mut cam_transform, horiz_coord) = camera_query.single_mut();
    let player_translation= player_query.single().translation;

    // Update camera orientation
    let cam_rotation = Quat::from_rotation_z(-horiz_coord.azimuth) * Quat::from_rotation_y(-horiz_coord.altitude);
    cam_transform.look_to(
        cam_rotation.mul_vec3(Vec3::X),
        cam_rotation.mul_vec3(Vec3::Z),
    );

    // Update camera position
    cam_transform.translation = player_translation - 10.0 * Vec3::from(cam_transform.forward());
}
