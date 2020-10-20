mod files;
mod filess;
mod settings;
mod data_types;
mod controls;

//use bevy::prelude::*;
use bevy::{
    prelude::*,
};

use files::*;
use filess::*;
use data_types::{sprite_sheet::SpriteSheet, general::*};

fn main() {
    
   App::build()
       .add_resource(settings::settings.system())
       .add_event::<ControlEvent>()
       .add_default_plugins()
       .add_startup_system(setup.system())
       .add_startup_system(asset_zombie.system())
       .init_resource::<KeyListener>()
       .init_resource::<ControlListener>()
       .add_system(controls::controls.system())
       .add_system(event_move.system())
       
       .run();
}
fn setup(  mut commands: Commands,){
      commands
        .spawn(Camera2dComponents {
            transform: Transform::from_scale(0.1),
            ..Default::default()
        })
        .spawn(UiCameraComponents::default())
    ;
}


fn event_move(

     mut controls_listen:ResMut<ControlListener>,
     mut dir:Local<Position>,
     mut sprite_index:Local<SpriteSheet>,
     control_events:Res<Events<ControlEvent>>,
    
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&Zombie, &mut Transform,&mut TextureAtlasSprite,&Handle<TextureAtlas>)>,
){
   //Uses events to get position to from controls
   
     for pos in controls_listen.control_event.iter(&control_events) {
  
      
    
    // gets the entity/spriteSheet
    
     for (monster, mut transform, mut texture_atlas_handle,texture) in &mut query.iter() {

        
       // let texture_atlas = texture_atlases.get(&texture).unwrap();
           
        println!("{:?}", pos.current_position.x);
        
        let translation = transform.translation_mut();
        // move the paddle horizontally
        *translation.x_mut() =  pos.current_position.x as f32;
        *translation.y_mut() =  pos.current_position.y as f32;
        texture_atlas_handle.index =sprite_index.Sprite_Sheet4x3().walk(dir.x,dir.y, pos.current_position.x,pos.current_position.y);
        //*translation.x_mut() += time.delta_seconds * x_direction * monster.speed;
        // bound the paddle within the walls
        //*translation.x_mut() = translation.x().min(380.0).max(-380.0);
        dir.x = pos.current_position.x;
        dir.y = pos.current_position.y;
    } 
} 
}
