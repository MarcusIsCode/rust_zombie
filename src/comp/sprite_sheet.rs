use crate::comp::data_types::*;

//TODO rename this SpriteSheet and make A new SpriteSheet and make a method for sprintSheet4x3
#[derive(Default)]
pub struct SpriteSheet{
  pub upp:u32,
  pub down:u32,
  pub left:u32,
  pub right:u32,
  pub index:u32,
}

pub struct SpriteSheet4x3 {
  pub sheet: SpriteSheet,
}
impl Default for SpriteSheet4x3 {
  fn default()->SpriteSheet4x3{
    SpriteSheet4x3{
      sheet:SpriteSheet{
          down:1,
          right:4, 
          upp:7, 
          left:10,
          index:1,
      }
    }
  }
}

impl SpriteSheet4x3{
 pub fn walk(&mut self,dir:Move)->u32{
    use Move::*;
   
    let mut _sprite_index = self.sheet.index;  
    //let mut compare_val:u32 = 1;
    
    match dir {
     
     LEFT  => _sprite_index =self.sheet.left,
     RIGHT => _sprite_index =self.sheet.right,
     DOWN  => _sprite_index =self.sheet.down,
     UPP   => _sprite_index =self.sheet.upp,

    }
    
 
     
    return  _sprite_index;
    /* //TODO add timing to make it look better maybe
     match moving {
            (comp,index) if index > comp  => { self.index = compare_val -1; println!("INDEX:{:?} more",self.index)},
            (comp,index) if index < comp => { self.index = compare_val; println!("INDEX:{:?} less",self.index)},
            (comp,index) if index == comp =>{  self.index = compare_val +1; println!("equal")},
      __=>(),
    } */
  
    
   // println!("compare_valll :::::{:?} INDEX:{:?}, moving::{:?}",compare_val, self.index, moving);

 } 
}