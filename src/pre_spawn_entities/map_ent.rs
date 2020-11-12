use bevy::{prelude::*,};
use crate::comp::{data_types::*};





pub fn map_setup(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>,
){

 let mut wall_thickens = 15.0;
 let wall_length_x= 1017.0;
 let mut wall_length_y = 710.0;

 let mut map_width = (500.0,350.0); 
 let map_offset =125.0;
 let wall_color = materials.add(Color::rgb(0.0, 0.0, 0.0).into()); 
  
for i in 0..4{
  if i ==1{
   
    map_width.0 = map_width.0 *-1.0;
    map_width.1 = map_width.1 *-1.0;
    wall_length_y = 715.0;
    wall_thickens = 20.0;
    
  }
  //left right wall
 commands
  .spawn(SpriteComponents {
            material:wall_color.clone(),
            transform: Transform::from_translation(Vec3::new(map_width.0 + map_offset,0.0, 0.00)),
            sprite: Sprite::new(Vec2::new(wall_thickens, wall_length_y)),
            ..Default::default()
        }).with(Wall)
          .with(Collider::SOLID);
//top bottom wall
  commands
  .spawn(SpriteComponents {
            material:wall_color.clone(),
            transform: Transform::from_translation(Vec3::new(map_offset -1.0,map_width.1 , 0.01)),
            sprite: Sprite::new(Vec2::new( wall_length_x, wall_thickens)),
            ..Default::default()
        }).with(Wall)
          .with(Collider::SOLID);
        
      }
      
      //floor
        commands
  .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.60, 0.60, 0.66).into()),
            transform: Transform::from_translation(Vec3::new(map_offset,0.0 , 0.0)),
            sprite: Sprite::new(Vec2::new( wall_length_x -wall_thickens, wall_length_y)),
            ..Default::default()
        });

  }


