/* #[allow(dead_code)]
fn hello_world(mut commands: Commands,asset_server: Res<AssetServer>) {
commands
.spawn(UiCameraComponents::default())
.spawn(TextComponents {
   style:Style{
         position_type:PositionType::Absolute,
         position:Rect{
            left:Val::Px(200.0),
            top:Val::Px(30.0),
            ..Default::default()

         },
         ..Default::default()
   },
        text: Text {
            value: "H".to_string(),
            font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
            style: TextStyle {
                font_size: 25.0,
                color: Color::WHITE,
            },
        },
        ..Default::default()
    });
}
struct Player;

fn setup(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            ..Default::default()
        })
        .with(Player);
}


 */