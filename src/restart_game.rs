use bevy::prelude::*;
use crate::comp::{data_types::*, characters::*};
//TODO
// add button
// make it do somthing like start the game

pub fn button_click(
   mut commands:Commands,
   mut game_info:ResMut<GameCounting>, 
   mut reset: Query<(&ResetButton,Mutated<Interaction>,&Children,)>,
   mut zombies: Query<(Entity,&Zombie)>,
   mut text:Query<(&mut Text, &GameText)>,
){
if game_info.player_hp.points == 0{  
    for (entity_zombie, _id_zombie) in zombies.iter_mut(){
        commands.despawn(entity_zombie);
    }


    for (_button, interaction,_children) in reset.iter_mut() {

        match *interaction {
            Interaction::Clicked=>{
                
                  
                    game_info.player_hp.points = 100;
                    game_info.kills = 0;
                   for (mut game_text, _text_id) in text.iter_mut(){
                       game_text.value = game_info.set_text();
                   } 
            },
            __=>{},
        }
    }
}
  
}