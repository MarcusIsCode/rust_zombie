use crate::comp::{characters::*,data_types::*};

use bevy::prelude::*;
pub struct ZombieInfo{
   pub atlas_handel:Handle<TextureAtlas>,
}

pub fn zombie_setup(
    asset_server: Res<AssetServer>,
    // storing the asset
    mut zombie_resource : ResMut<Zombie>,
    //storing Sprite sheet Asset
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    
){
 
      // loads the texture and gives it an handel that results in a uniq id
      // in load sync it stores it in texture_assets
    let sprite_handel =  asset_server.load("../assets/sprites/zombies/zombi1.png");
    let sprite_atlas = TextureAtlas::from_grid(sprite_handel,Vec2::new(20.0,20.0), 3, 4);          
        zombie_resource.atlas_handel = texture_atlases.add(sprite_atlas);
        zombie_resource.atlas_loaded = true;
      
      }
    


pub fn zombie_spawner(
    mut commands: Commands,
    time:Res<Time>,
    zombie_resource: Res<Zombie>,
    mut zombie_resource_update : ResMut<Zombie>
  ){
   zombie_resource_update.spawn_time.tick(time.delta_seconds); 
   let incoming_zombie:bool = zombie_resource.spawn_time.finished;

  if zombie_resource.atlas_loaded && incoming_zombie{
    
     let zombie_handle = zombie_resource.atlas_handel.clone_weak();
      commands
       .spawn(SpriteSheetComponents{
              texture_atlas:zombie_handle,
               transform:Transform::from_matrix(
                                Mat4::from_scale_rotation_translation(
                                     Vec3::splat(4.0),
                                  Default::default(),
                                     Vec3::new(0.0,50.0,1.0)
                              )
                          ),
              ..Default::default()
          })
             .with(Zombie {..Default::default()})
             .with(Timer::from_seconds(0.5,true))  
             .with(Collider::BREAKABLE);
  }
 


}

           
    