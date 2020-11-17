use bevy::prelude::*;
use crate::comp::{data_types::*, weapons::*,data_types_event::*};
 #[derive(Default,Debug)]
 //TODO include this in shoot
 pub struct SaveEnt{
  
    tmp_save:Vec<Entity>,
    
 }

pub fn shoot_event_move (
    mut commands:Commands,
       
    mut game_counting_update: ResMut<GameCounting>,

   
    mut shoot_send:ResMut<Events<ShootEvent>>,
    mut shoot_store_new:ResMut<Shoot>,
  mut shoot_query: Query<(Entity, &Shoot, &mut Transform ,&mut Timer)>, 

){
   

    
         for (ent,shoot_info, mut transform,timer) in &mut shoot_query.iter_mut(){
             let pos = &mut transform.translation;
               
                *pos.x_mut() +=  (10*shoot_info.direction.0) as f32;
                *pos.y_mut() +=  (10*shoot_info.direction.1) as f32;     
             
                    
                if timer.finished{
               
                     commands.despawn(ent);
                  
                     game_counting_update.decrees_shoots();
                    
                       shoot_send.send(ShootEvent{
                            action:Action::HOLD,
                            direction:shoot_info.direction,
                        }); 
                }

    
            
        }
  
}