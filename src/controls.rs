use bevy::prelude::*;
use crate::types::InputEvent;


const SPEED: f32 = 3.0; // m/s
const ANGULAR_SPEED: f32 = 1.0; // rad/s


pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputEvent>();
        app.add_systems(PreUpdate, dispatch_input);
    }
}

fn dispatch_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut event_output: EventWriter<InputEvent>,
    mut exit:         EventWriter<bevy::app::AppExit>,
) {
    if keys.pressed(KeyCode::KeyW) {
        event_output.send(InputEvent::MoveForward(SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::KeyA) {
        event_output.send(InputEvent::MoveLeft(SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::KeyS) {
        event_output.send(InputEvent::MoveForward(-SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::KeyD) {
        event_output.send(InputEvent::MoveLeft(-SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::Space) {
        event_output.send(InputEvent::MoveUp(SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        event_output.send(InputEvent::MoveUp(-SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::ArrowUp) {
        event_output.send(InputEvent::TurnUp(ANGULAR_SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        event_output.send(InputEvent::TurnLeft(ANGULAR_SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::ArrowDown) {
        event_output.send(InputEvent::TurnUp(-ANGULAR_SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::ArrowRight) {
        event_output.send(InputEvent::TurnLeft(-ANGULAR_SPEED * time.delta_seconds()));
    }
    if keys.pressed(KeyCode::Escape) {
        exit.send(bevy::app::AppExit);
    }
}
