use crate::comp::{characters::*,data_types::*};

use bevy::prelude::*;


pub fn zombie_spawner(
    mut commands: Commands,
    time:Res<Time>,
    zombie_resource: Res<Zombie>,
    mut game_info:ResMut<GameCounting>, 
    mut zombie_resource_update : ResMut<Zombie>
  ){
      zombie_resource_update.spawn_time.tick(time.delta_seconds); 
  
  let incoming_zombie:bool = zombie_resource.spawn_time.finished;

  
  if zombie_resource.atlas_loaded && incoming_zombie && game_info.player_hp.points != 0 {
    
     let zombie_handle = zombie_resource.atlas_handel.clone_weak();
      commands
       .spawn(SpriteSheetComponents{
              texture_atlas:zombie_handle,
               transform:Transform::from_matrix(
                                Mat4::from_scale_rotation_translation(
                                     Vec3::splat(4.0),
                                  Default::default(),
                                    StartSpawn{..Default::default()}.zombie_spawn_position(),
                              )
                          ),
              ..Default::default()
          })
             .with(Zombie {..Default::default()})
             .with(Timer::from_seconds(0.5,true))  
             .with(Collider::BREAKABLE);
  }
 


}