use crate::comp::{characters::*,HUMANOID_SIZE};

use bevy::prelude::*;


pub fn zombie_setup(
    asset_server: Res<AssetServer>,
    // storing the asset
    mut zombie_resource : ResMut<Zombie>,
    //storing Sprite sheet Asset
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    
){
 
    //TODO load folder with zombie sprites
    let sprite_handel =  asset_server.load("../assets/sprites/zombies/zombi1.png");
    let sprite_atlas = TextureAtlas::from_grid(
                                      sprite_handel,
                                      HUMANOID_SIZE,
                                       3, 4
                                      );          
        zombie_resource.atlas_handel = texture_atlases.add(sprite_atlas);
        zombie_resource.atlas_loaded = true;
      
      }
    




           
    