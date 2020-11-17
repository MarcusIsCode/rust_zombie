use bevy::math::*;

use crate::data_types::Map;
pub const WINDOW_SIZE:Size<u32> = Size{
    width:1080,
    height:900,
};

pub const MAP_BORDERS:Map = Map{
    max_min_x:(-355, 605),
    max_min_y:(-310, 340),
};
pub const HUMANOID_SIZE:Vec2 = const_vec2!([23.0,20.0]);

pub fn map_boundaries(position:(i32,i32))->(i32,i32){
    let x_borders = MAP_BORDERS.max_min_x; 
    let y_borders = MAP_BORDERS.max_min_y;
   
    match position{
    (x,_) if x <= x_borders.0 => (x_borders.0, position.1),    
    (x,_) if x >= x_borders.1 => (x_borders.1, position.1), 

    (_,y) if y <= y_borders.0 =>(position.0, y_borders.0),
    (_,y) if y >= y_borders.1=> (position.0, y_borders.1), 

     __=>position,
    }

       
}
