//TODO rename this SpriteSheet and make A new SpriteSheet and make a method for sprintSheet4x3
#[derive(Default)]
pub struct SpriteSheet{
 upp:u32,
 down:u32,
 left:u32,
 right:u32,
 index:u32,
}

impl SpriteSheet  {
  pub fn Sprite_Sheet4x3(&mut self) ->SpriteSheet {
     SpriteSheet{
            down:1,
            right:4, 
            upp:7, 
            left:10,
            index:1,
      }
  }
}
impl SpriteSheet{
 pub fn walk(&mut self,dir_x:i32,dir_y:i32, pos_x:i32, pos_y:i32)->u32{
    let mut compare_val:u32 = 1;
    let left_right = (dir_x,pos_x);
    let upp_down = (dir_y,pos_y);

   
    
    match left_right {
      (d,p) if d == p =>(),
      (d,p) if d > p => compare_val = self.left,
      (d,p) if d < p => compare_val = self.right,
      __=>(),
    }
    match upp_down {
      (d,p) if d == p =>(),
      (d,p) if d > p => compare_val = self.down,
      (d,p) if d < p => compare_val = self.upp,
      __=>(),
    }
     let moving =(compare_val,self.index);
    self.index = compare_val;
    /* //TODO add timing to make it look better maybe
     match moving {
            (comp,index) if index > comp  => { self.index = compare_val -1; println!("INDEX:{:?} more",self.index)},
            (comp,index) if index < comp => { self.index = compare_val; println!("INDEX:{:?} less",self.index)},
            (comp,index) if index == comp =>{  self.index = compare_val +1; println!("equal")},
      __=>(),
    } */
  
    
    println!("compare_valll :::::{:?} INDEX:{:?}, moving::{:?}",compare_val, self.index, moving);
   return self.index as u32;
 } 
}