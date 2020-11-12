use bevy::prelude::*;
use crate::data_types::{Speed,Move,StartSpawn};
#[derive(Default,Debug,Copy, Clone)]
pub struct Health{
  pub points:u32,
}
pub struct Player{
   pub name:String,
   pub speed:Speed,
   pub move_action:Move,
   pub pos:(i32,i32),
   pub direction:(i32,i32),
}
impl Default for Player {
  fn default()->Player{
     Player{
       name:"Player 1".to_string(),
       speed:Speed{velocity:5},
       move_action:Move::DOWN,
       pos:(0,0),
       direction:(Move::DOWN.direction()),
       
     }
  }
}
#[derive(Debug)]
pub struct Zombie{
   pub speed:Speed,
   pub direction:(i32,i32),
   pub atlas_handel:Handle<TextureAtlas>,
   pub atlas_loaded:bool,
   pub spawn_time:Timer,
}
impl Default for Zombie {
  fn default()->Zombie{
     Zombie{
       speed:Speed{velocity:3},
       direction:(-1,0),
       atlas_handel:Default::default(),
       atlas_loaded:false,
       spawn_time:Timer::from_seconds(9.0, true)
     }   
  }
}
//TODO include in wall or object 
 pub fn move_to_player(player_dir:(i32,i32), zombie_dir:(i32,i32))->(i32,i32){
    let x_compare = zombie_dir.0 - player_dir.0;
    let y_compare = zombie_dir.1 - player_dir.1;
  
    let xy = (x_compare, y_compare);
    let res;
    //To make sure it walks in one direction at the time
    match  xy {
      (x,y) if x <= 0 && x < y => {
         res = (1,0);
      },
      (x,y) if x <= 0 && x >y =>{
        res = (0,1);
      }
      (x,y) if x > 0 && x > y =>{
        res  = (-1,0);
      }
      (x,y) if x > 0 && x <y =>{
        res = (0,-1);
      }
      __=> res = (1,0)
    } 

     

    return res
 }

