use bevy::{prelude::*, sprite::collide_aabb::{collide}};
use crate::comp::*;


pub fn shoot_collision(  
    mut commands: Commands,
   
    mut game_ev:ResMut<GameCounting>,
    mut text:Query<(&mut Text, &GameText)>,
    mut shoot_collide:Query<(Entity, &Shoot,&Transform, &Sprite, &mut Timer )>,
     zombie_query: Query<(&Zombie, &Collider, &Transform, Entity)>
){
    
       
    
        for (_ent, _shoot_info, transform_s, sprite, mut timer) in &mut  shoot_collide.iter_mut(){
            let shoot_size = sprite.size;
            //println!("transform_s{:?}, shoot_size:{:?}", transform_s.translation(),shoot_size );
            for (_zombie_info, _zombie_col, transform_z, _ent_z) in &mut zombie_query.iter(){
                 //println!("transform_z{:?}", transform_z.translation());
                 
                let collision = collide(
                    transform_s.translation,
                    shoot_size,
                    transform_z.translation,
                    Vec2::new(22.0, 22.0),
                );
                
                if let Some(_collision) = collision{
                    commands.despawn(_ent_z);
                    commands.despawn(_ent);
                    timer.finished = true;

                    game_ev.kills +=1;
                    for (mut game_text, __ent) in text.iter_mut(){
                        game_text.value = game_ev.set_text();
                       
                    }
                   
                    break;

                }
            }
    
        }
       
    }