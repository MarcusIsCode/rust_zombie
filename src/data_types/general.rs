use bevy::{
    prelude::*,
    input::keyboard::KeyboardInput,
};

pub struct Zombie{
   pub speed:f32,
}
#[derive(Default)]
pub struct Position{
    pub x:i32,
    pub y:i32,
}


#[derive(Default)]
//Events
pub struct KeyListener {
  pub event_reader: EventReader<KeyboardInput>,
}

pub struct ControlEvent {
  pub current_position:Position,
}
#[derive(Default)]
pub struct ControlListener{
   pub control_event:EventReader<ControlEvent>,
}