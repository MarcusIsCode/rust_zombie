use bevy::{prelude::*};
use crate::data_types::*;

#[derive(Debug,Copy, Clone)]
pub struct Damage{
  hit_points:u32,
}

pub struct Shoot {
    pub size:Vec2,
    pub direction:(i32,i32),
    pub start_point:StartSpawn,
    pub action: Action,
}

impl Default for Shoot {
    fn default() -> Shoot{
        Shoot{
            size:Vec2::new(5.0,20.0),
            direction:(0,-1),
            start_point:StartSpawn(0,0),
          action:Action::HOLD,
        }
    }
}

  pub fn shoot_size(dir:(i32,i32))->Vec2{
      let mut shot_size = Vec2::new(5.0, 20.0);
        

       if dir.0 != 0 {
         shot_size = Vec2::new(20.0, 5.0);
       }

        return shot_size;
    }
