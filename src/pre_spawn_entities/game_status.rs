use bevy::prelude::*;
use crate::comp::data_types::*;


pub fn game_status_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_info:ResMut<GameCounting>,
    mut materials:ResMut<Assets<ColorMaterial>>,
    ) {
    let font_handle = asset_server.load("../assets/fonts/FiraSans-Bold.ttf").clone();
    
    let game_text = game_info.set_text();
  
    commands
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

    commands.spawn(ButtonComponents{
        style:Style{
            size:Size::new(Val::Px(100.0),Val::Px(50.0)),
            position_type:PositionType::Absolute,
            position:Rect{
                top:Val::Px(200.0),
                left:Val::Px(50.0),
                ..Default::default()
            },
            justify_content: JustifyContent::Center,
               
            align_items: AlignItems::Center,
            ..Default::default()
        },
            material:materials.add(Color::GREEN.into()),
            ..Default::default()
         
        }
        
     ).with(ResetButton)
     .with_children(|parent|{
           parent.spawn(TextComponents {
                text: Text {
                    value: "Reset".to_string(),
                    font:  asset_server.load("../assets/fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                },
                ..Default::default()
            });
        });

}


