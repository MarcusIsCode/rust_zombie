use bevy::prelude::{AppBuilder,Plugin};

pub mod sprite_sheet;
pub mod data_types;
pub mod data_types_event;
pub mod characters;
pub mod load;
pub mod weapons;
pub mod static_data;

pub use self::{
    sprite_sheet::*,
    data_types::*,
    data_types_event::*,
    characters::*,
    sprite_sheet::*,
    load::*,
    weapons::*,
    static_data::*,
};
pub struct ResourcesSetup;

impl Plugin for ResourcesSetup {
    fn build(&self, app: &mut AppBuilder){
        app
           .add_resource(Player{..Default::default()})
           .add_resource(GameCounting{player_hp:Health{points:100},..Default::default()})
           .add_resource(Shoot{..Default::default()})
           .add_resource(Zombie{..Default::default()});
         
    }
}

