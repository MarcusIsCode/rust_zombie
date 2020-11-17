use bevy::prelude::*;
pub mod player_ent;

pub mod map_ent;
pub mod game_status;
pub use self::{
    player_ent::*,
  
    map_ent::*,
    game_status::*,
};

pub struct  EntitiesSetup;

impl Plugin for EntitiesSetup{
    fn build(&self,app: &mut AppBuilder){
        app
            .add_startup_system(setup.system())
            .add_startup_system(map_setup.system())
            .add_startup_system(player_setup.system())        
            .add_startup_system(game_status_text.system());
          
    }
}
pub fn setup(  mut commands: Commands){

    commands
       .spawn(Camera2dComponents::default())
       .spawn(UiCameraComponents::default());
}
