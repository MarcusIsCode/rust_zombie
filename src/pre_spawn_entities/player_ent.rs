use crate::comp::characters::*;
use crate::data_types::Collider;
use crate::static_data::HUMANOID_SIZE;
use bevy::prelude::*;




pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
  
  

     let sprite_handel =  asset_server.load("../assets/sprites/player/playerSpriteSheet.png");
     let sprite_atlas = TextureAtlas::from_grid(sprite_handel,HUMANOID_SIZE, 3, 4);
     let player_texture_handel = texture_atlases.add(sprite_atlas);
             
   
     commands
       .spawn(SpriteSheetComponents{
              texture_atlas:player_texture_handel,
              transform:Transform::from_matrix(
                                Mat4::from_scale_rotation_translation(
                                    Vec3::splat(4.0) , 
                                    Default::default(),
                                    Vec3::new(0.0,0.0,1.1)
                                )
                            ),
              ..Default::default()
          })
             .with(Player {..Default::default()})
             .with(Collider::BREAKABLE);
  
     
}

