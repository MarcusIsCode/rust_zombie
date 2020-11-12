use bevy::prelude::*;
use crate::comp::{data_types::*, weapons::*,data_types_event::*};
 #[derive(Default,Debug)]
 //TODO include this in shoot
 pub struct SaveEnt{
  
    tmp_num:i32,
    
 }

pub fn shoot_event_move (
    mut commands:Commands,
       
    game_counting: Res<GameCounting>,
    mut game_counting_update: ResMut<GameCounting>,
    mut save_ent: Local<SaveEnt>,
  
   
    mut shoot_send:ResMut<Events<ShootEvent>>,
    mut shoot_store_new:ResMut<Shoot>,
  mut shoot_query: Query<(Entity, &Shoot, &mut Transform)>, 

){

  
   if game_counting.shoots >0{
    
         for (ent,shoot_info, mut transform) in &mut shoot_query.iter_mut(){
            
                let pos = &mut transform.translation;
               
                 
                 
                if  save_ent.tmp_num != 200 {
                   
                    
                        *pos.x_mut() +=  (2*shoot_info.direction.0) as f32;
                    
                        *pos.y_mut() +=  (2*shoot_info.direction.1) as f32;
                          //  println!("entnum {}", save_ent.tmp_num);
                        save_ent.tmp_num+= 1;

                    if save_ent.tmp_num == 200 {
                        save_ent.tmp_num = 0;
                        commands.despawn(ent);
                       
                        shoot_store_new.action = Action::HOLD;
                        game_counting_update.decrees_shoots();
                        shoot_send.send(ShootEvent{
                            action:Action::HOLD,
                            direction:shoot_info.direction,
                        })
                         
                    }
                    
                }
             
            } 
        }
  
}