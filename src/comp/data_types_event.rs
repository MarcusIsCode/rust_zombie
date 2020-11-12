use bevy::{
    prelude::*,
    input::keyboard::KeyboardInput,
};
use crate::comp::data_types::*;
//controls
#[derive(Default)]
pub struct KeyListener {
  pub event_reader: EventReader<KeyboardInput>,
}
#[derive(Debug)]
pub struct ControlEvent {
  pub current_move:Move,
  pub current_action:Action,
}

#[derive(Default)]
pub struct ControlListener{
   pub control_event:EventReader<ControlEvent>,
}

//shoot
pub struct ShootEvent{
  pub action:Action,
  pub direction:(i32,i32),
}
impl Default for ShootEvent {
  fn default() -> ShootEvent{
    ShootEvent{
      action:Action::HOLD,
      direction:(0,0),
    }
  }
}

#[derive(Default)]
pub struct  ShootEventListener{
    pub shoot_event:EventReader<ShootEvent>,
}