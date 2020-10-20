pub struct PathUI{
    pub font_path:String,
}
pub struct AssetsLoad{
    pub path:String,
    pub path_zombie:String,
    pub path_player:String,
    pub path_ui:PathUI,
}
impl Default for AssetsLoad {
    fn default()->AssetsLoad{
        AssetsLoad{
            path:"assets/".to_string(),
            path_zombie:"assets/sprites/zombies".to_string(),
            path_player:"assets/sprites/player".to_string(),
            path_ui:PathUI{
                font_path:"assets/fonts".to_string(),
            }
        }
    }
}

