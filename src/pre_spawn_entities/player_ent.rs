use crate::comp::characters::*;
use crate::data_types::Collider;
use bevy::prelude::*;




pub fn player_setup(
    mut commands: Commands,
    //used to get the assets
    asset_server: Res<AssetServer>,
    // storing the asset
    // mut textures_assets: ResMut<Assets<Texture>>,
    //storing Sprite sheet Asset
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
  
  
      // loads the texture and gives it an handel that results in a uniq id
      // in load sync it stores it in texture_assets
     let sprite_handel =  asset_server.load("../assets/sprites/player/playerSpriteSheet.png");
   
    //let sprite_size = textures_assets.get(&sprite_handel).unwrap().size;

     let sprite_atlas = TextureAtlas::from_grid(sprite_handel, Vec2::new(20.0,20.0), 3, 4);
     
     //we store it among in Assets textures
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