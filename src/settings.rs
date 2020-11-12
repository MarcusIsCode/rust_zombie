use bevy::{prelude::*, window::WindowMode};
use crate::comp::static_data::WINDOW_SIZE;

fn window()-> WindowDescriptor
{
WindowDescriptor{
    width:WINDOW_SIZE.width,
    height:WINDOW_SIZE.height,
    title:String::from("Rust Zombie"),
    vsync:true,
    resizable:false,
    mode:WindowMode::Windowed, 
   ..Default::default()
 }
}

pub fn settings(){
   window();
}
