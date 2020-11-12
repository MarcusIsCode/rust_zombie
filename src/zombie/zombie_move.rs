use crate::comp::{data_types::*, characters::*, sprite_sheet::*};                     
use bevy::prelude::*;
pub fn zombie_moving(

mut sprite_index:Local<SpriteSheet4x3>,
player:Res<Player>,
mut query_walk:Query<(&mut TextureAtlasSprite,&Zombie,&mut Transform, &mut Timer,  )>
){
    
 for (mut texture, _zombie, mut transform,timer ) in  query_walk.iter_mut(){
     let pos  = &mut transform.translation;
  
     let zombie_move_dir = move_to_player(player.pos, (*pos.x_mut() as i32, *pos.y_mut() as i32));

        if  timer.finished {
           
                texture.index = sprite_index.walk(Move::direction_reverse(zombie_move_dir));
                *pos.x_mut() += (1 * zombie_move_dir.0) as f32; 
                *pos.y_mut() += (1 * zombie_move_dir.1) as f32;
        }
       

    }

}