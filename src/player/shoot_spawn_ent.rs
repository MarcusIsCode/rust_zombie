// TODO clean upp

use bevy::prelude::*;
use crate::comp::{data_types::*, weapons::*,data_types_event::*,characters::*};

pub fn shoot_event_spawn(
    mut commands:Commands,
     
    mut materials:ResMut<Assets<ColorMaterial>>,
    mut game_counting: ResMut<GameCounting>,
        player:Res<Player>,
     
    mut shoot_listen:Local<EventReader<ShootEvent>>,
        shoot_events:Res<Events<ShootEvent>>,
   
    
    ){
    

for shoot in  shoot_listen.iter(&shoot_events){
            if let Action::SHOOT = shoot.action{
                if game_counting.shoots < 10{
                    let shoot_info = Shoot{
                                        size:shoot_size(shoot.direction),
                                        direction:shoot.direction,
                                        action:shoot.action,
                                        start_point:StartSpawn(player.pos.0,player.pos.1),
                    };  
                 
                  commands
                     .spawn(shoot_info.shoot_to_sprite(materials.add(Color::GREEN.into())))
                               .with(shoot_info)
                               .with(Timer::from_seconds(1.0, false)) ;
                        
                         game_counting.shoots+=1;
                         break;
                }
    
     }
    }
}
