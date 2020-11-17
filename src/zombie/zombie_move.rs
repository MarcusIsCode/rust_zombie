use crate::comp::{data_types::*, characters::*, sprite_sheet::*};                     
use bevy::{prelude::*, sprite::collide_aabb::{collide}};

pub fn zombie_moving(

mut sprite_index:Local<SpriteSheet4x3>,
mut game_ev:ResMut<GameCounting>,
    player:Res<Player>,
    
    
mut query_walk:Query<(&mut TextureAtlasSprite,&Zombie,&mut Transform, &mut Timer, )>,
mut player_query: Query<(&mut TextureAtlasSprite,&Player ,&mut Transform)>,
mut text:Query<(&mut Text, &GameText)>,
){
    
 for (mut texture, _zombie, mut transform,timer ) in  query_walk.iter_mut(){
     let pos  = &mut transform.translation;
  
     let zombie_move_dir = move_to_player(player.pos, (*pos.x_mut() as i32, *pos.y_mut() as i32));

        if  timer.finished {
           
                texture.index = sprite_index.walk(Move::direction_reverse(zombie_move_dir));
                *pos.x_mut() += (1 * zombie_move_dir.0 * _zombie.speed.velocity) as f32; 
                *pos.y_mut() += (1 * zombie_move_dir.1* _zombie.speed.velocity) as f32;
        }
       for (_texture, _player, mut player_transform ) in player_query.iter_mut(){

            let attack = collide(  
                player_transform.translation,
                Vec2::new(22.0, 22.0),
                transform.translation,
                Vec2::new(22.0, 22.0),
            );

            if let Some(_colilsion) = attack{
                if game_ev.player_hp.points != 0{
                    game_ev.player_hp.points -=1
                };
                for (mut game_text, _GameText,) in text.iter_mut(){
                    game_text.value = game_ev.set_text();
                }
            }

       }

    }

}

