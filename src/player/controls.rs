use bevy::{
    prelude::*,
    input::{ElementState ,keyboard::KeyboardInput},
};
use crate::comp::static_data::map_boundaries;
use crate::comp::{data_types::*,characters::*, data_types_event::*,weapons::*};


pub fn controls(
     //player resourse
     player:Res<Player>,
     mut player_new:ResMut<Player>,
     
      shoot_res:Res<Shoot>,
    

     // bevy key events
     keyboard_input_events: Res<Events<KeyboardInput>>,
     mut state: ResMut<KeyListener>,
     
      //player control events
     mut control_event:ResMut<Events<ControlEvent>>,
     
     //shoot Events
     mut shoot_event:ResMut<Events<ShootEvent>>,
   
) {

for event in state.event_reader.iter(&keyboard_input_events) {
 use Action::*;
 use Move::*;  
 
 let key:Option<KeyCode> = event.key_code;
 

 if event.state == ElementState::Pressed{
      
         match key{
          Some(KeyCode::A)=>{
         
            player_new.move_action  = LEFT;

            player_new.pos.0 += player.move_action.moving_with_speed(player.speed).0;
            
            player_new.direction = player.move_action.direction();
            
          },
          Some(KeyCode::D)=>{  
            player_new.move_action = RIGHT;
            player_new.pos.0 += player.move_action.moving_with_speed(player.speed).0;
            
            player_new.direction = player.move_action.direction();
             
          },
          Some(KeyCode::W)=>{
             
              player_new.move_action= UPP;
               player_new.pos.1 += player.move_action.moving_with_speed(player.speed).1;

              player_new.direction =  player.move_action.direction();
             
          },
          Some(KeyCode::S)=>{
                player_new.move_action = DOWN;
                player_new.pos.1 += player.move_action.moving_with_speed(player.speed).1;
                player_new.direction =  player.move_action.direction();
              
          },
          Some(KeyCode::Space)=>{
                player_new.direction =  player.move_action.direction();
                  if let HOLD = shoot_res.action{

                                shoot_event.send(ShootEvent{
                                            action:SHOOT,
                                            direction:player.direction,
                                        })
                                  }             
          }
          None =>(),
          __=>(),
            
         }
         player_new.pos =map_boundaries(player_new.pos);
         println!("dX:{},dY:{}, action {:?}", player.pos.0,player.pos.1,player.move_action);

      }
  
        control_event.send(ControlEvent{
          current_move:player.move_action,
          current_action:shoot_res.action});
    }     

}