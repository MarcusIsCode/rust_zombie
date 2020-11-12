use bevy::prelude::{AppBuilder,Plugin,IntoQuerySystem};
pub mod player_move;
pub mod shoot_move;
pub mod controls;
pub mod shoot_spawn_ent;

pub use self::{
   shoot_spawn_ent::*,
   player_move::*,
   shoot_move::*,
   controls::*,
 
};
pub struct GameSystemSetup;

impl Plugin for GameSystemSetup{
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(controls.system())
            .add_system(event_move.system())
            .add_system(shoot_event_spawn.system())
            .add_system(shoot_event_move.system());
    }
            
           
}

