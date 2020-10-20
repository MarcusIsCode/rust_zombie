use crate::data_types::*;
use bevy::prelude::*;





pub fn asset_zombie(
    mut commands: Commands,
    //used to get the assets
    asset_server: Res<AssetServer>,
    // storing the asset
    mut textures_assets: ResMut<Assets<Texture>>,
    mut assets_sprites: ResMut<Assets<TextureAtlas>>,
){
  
      let asset_folder = asset_server.load_asset_folder("assets/sprites/zombies/").unwrap();
      println!("{:?}",asset_folder);

      let zombie_1 =  asset_server
            .load_sync(
                        &mut textures_assets,
                        "assets/sprites/zombies/zombi1.png")
            .unwrap();
            
     let zombie_1_texture = textures_assets.get(&zombie_1).unwrap();
     let zombie_1_sheet = TextureAtlas::from_grid(zombie_1, zombie_1_texture.size, 3, 4);
     
     let zombie_1_texture_handel = assets_sprites.add(zombie_1_sheet);
             
   
     commands
     
          .spawn(SpriteSheetComponents{
              texture_atlas:zombie_1_texture_handel,
               transform: Transform::from_scale(0.15),
              ..Default::default()
          })
             .with(Zombie {
            speed: 500.0,
        });
  
     
}
