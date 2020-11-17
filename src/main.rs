mod settings;
mod restart_game;
mod comp;
mod player;
mod pre_spawn_entities;
mod zombie;
mod colilsion;
//use bevy::prelude::*;
use bevy::{ecs::*, prelude::*, render::pass::ClearColor, sprite::collide_aabb::{collide, Collision}};
use pre_spawn_entities::*;
use player::*;
use zombie::*;
use comp::*;
use colilsion::*;

fn main() {
    
   App::build()
       .add_event::<ControlEvent>()
       .add_event::<ShootEvent>()
       .add_resource(settings::settings.system())
       .add_plugin(ResourcesSetup)
       .init_resource::<KeyListener>()
       .init_resource::<ControlListener>()
       .init_resource::<ShootEventListener>()
       .add_plugins(DefaultPlugins)
       
       .add_plugin(EntitiesSetup)
       .add_startup_system(zombie_setup.system())
       .add_plugin(GameSystemSetup)
       .add_system(restart_game::button_click.system())
       .add_system(shoot_collision.system())
       .add_system(zombie_spawner.system())
       .add_system(zombie_moving.system()) 
       .run(); 
}







     


