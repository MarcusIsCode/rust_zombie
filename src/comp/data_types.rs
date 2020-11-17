use crate::comp::characters::Health;
use crate::static_data::*;
use bevy::prelude::*;
use rand::Rng;

pub struct ResetButton;
#[derive(Default,Debug,Copy, Clone)]
pub struct Speed{
  pub velocity:i32,
}
#[derive(Default,Debug,Copy, Clone)]
pub struct GameCounting{
  pub shoots:u32,
  pub kills:u32,
  pub player_hp:Health, 
}
pub struct GameText;

impl GameCounting {
  pub fn decrees_shoots (&mut self){
    if self.shoots != 0{
      //temp fix
      self.shoots =0;
    } 
    
  }
  pub fn set_text(&mut self)->String{
    let hp = self.player_hp.points.to_string();
    let score = self.kills.to_string();
    return format!("Health:{}   Kills:{}",hp, score);
  }
  
}
#[derive(Default,Debug,Copy, Clone)]
pub struct StartSpawn(pub i32,pub i32);
impl StartSpawn{
  fn turn_vec3(&mut self)->Vec3{
    return  Vec3::new(self.0 as f32, self.1 as f32, 1.0);
  }

  pub fn zombie_spawn_position(&mut self)->Vec3{
    let range_x = MAP_BORDERS.max_min_x;
    let range_y = MAP_BORDERS.max_min_y;
    let mut rng = rand::thread_rng();

    
    self.0 = rng.gen_range(range_x.0 *-1,range_x.1) * rand_not_zero(); 
    self.1 = rng.gen_range(range_y.0 *-1,range_y.1) * rand_not_zero(); 
 

    return self.turn_vec3() ;
  }

}
pub fn rand_not_zero()->i32{
     if rand::random(){
         return  1;
       }else{
         return -1;
       }
  }
pub struct Map{
  pub max_min_x:(i32,i32),
  pub max_min_y:(i32,i32)
}
pub struct Wall;


#[derive(Debug,Copy, Clone)]
pub enum Action {
  SHOOT,
  RELOAD,
  HOLD,
}
pub enum Collider {
  SOLID,
  BREAKABLE,
}

pub enum _Hit {
  DAMAGE,
  MISS
}
#[derive(Debug,Copy, Clone)]
pub enum Move{
  UPP ,
  DOWN ,
  LEFT ,
  RIGHT,
} 
 impl Move {
 pub fn direction(&self)->(i32,i32){
  use Move::*;  
  let res:(i32,i32);
    match &self {
      UPP => res = (0,1),
      DOWN =>res = (0,-1),
      LEFT => res = (-1,0),
      RIGHT => res = (1,0),
    }
    return res;
  }
 pub fn direction_reverse(dir:(i32,i32))->Move{
    //TODO make a fail safe if the number if is bigger then 1 and less then -1,
    match dir {
      (0,1)=> Move::UPP,
      (0,-1)=> Move::DOWN,
      (-1,0)=> Move::LEFT,
      (1,0) => Move::RIGHT,
      __=>Move::DOWN,
    }

 }
pub fn moving_with_speed(&self, speed:Speed)->(i32,i32){
    let mut moving_speed:(i32,i32) = self.direction() ; 
            moving_speed.0 = moving_speed.0 * speed.velocity;
            moving_speed.1 = moving_speed.1 * speed.velocity;
      
            return moving_speed;
  }
}

 