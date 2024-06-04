use bevy::prelude::*;


#[derive(Event, Clone, Copy)]
pub enum InputEvent {
    MoveForward(f32),
    MoveLeft(f32),
    MoveUp(f32),
    TurnLeft(f32),
    TurnUp(f32),
}
