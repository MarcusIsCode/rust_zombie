use bevy::prelude::*;
use crate::comp::*;


pub fn event_move(
      //spriteSheet with methtods for handling spritesheets
    mut sprite_index:Local<SpriteSheet4x3>,
     
      // takes the position data from the controll evenet
    control_events:Res<Events<ControlEvent>>,
    mut controls_listen:Local<EventReader<ControlEvent>>,
    player:Res<Player>,
   
    mut player_query: Query<(&mut TextureAtlasSprite,&Player ,&mut Transform)>,
){
   //Uses events to get position to from controls
   
    for control in controls_listen.iter(&control_events) {
       
            
      
      for ( mut texture_atlas_handle,_player  ,mut transform) in  player_query.iter_mut() {
     
        let speed = player.pos;

            
        let translation = &mut transform.translation;
             
             *translation.x_mut() =  speed.0 as f32;
            
             *translation.y_mut() =  speed.1 as f32;
             texture_atlas_handle.index = sprite_index.walk(control.current_move);
            
             
           

      } 
    } 
}