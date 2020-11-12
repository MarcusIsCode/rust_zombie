use bevy::prelude::*;
use crate::comp::data_types::*;



pub fn game_status_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_info:ResMut<GameCounting>,
    ) {
    let font_handle = asset_server.load("../assets/fonts/FiraSans-Bold.ttf");
    
    let game_text = game_info.set_text();
  
    commands
     
        // texture
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: game_text,
                font: font_handle,
                style: TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        }).with(GameText);
}
