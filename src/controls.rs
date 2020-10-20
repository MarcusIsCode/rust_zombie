use bevy::{
    prelude::*,
    input::keyboard::{KeyboardInput, ElementState},
};

use crate::data_types::*;


pub fn controls(
    mut position:Local<Position>,
  
     keyboard_input_events: Res<Events<KeyboardInput>>,//trigger,
    mut state: ResMut<KeyListener>,//event
    mut control_event:ResMut<Events<ControlEvent>>,

) {

for event in state.event_reader.iter(&keyboard_input_events) {
     
 let key:Option<KeyCode> = event.key_code;


 if event.state == ElementState::Pressed{
   
         match key{
          Some(KeyCode::A)=>{
            position.x-=2;
     
          },
          Some(KeyCode::D)=>{  
            position.x+=2;
             
          },
          Some(KeyCode::W)=>{
            
             position.y+=2;
       
          },
          Some(KeyCode::S)=>{
             position.y-=2;
          
               
          },
         
          None =>(),
          _ => (),
        }
         println!("X:{},Y:{}",position.x,position.y);
       
        control_event.send(ControlEvent{
          current_position:Position{
                                      x:position.x,
                                      y:position.y
                                    },
         
        })
         
      }
    }

}