use bevy::{prelude::*, window::WindowMode};


fn window()-> WindowDescriptor
{
WindowDescriptor{
    width:500,
    height:700,
    title:String::from("Rust Zombie"),
    vsync:true,
    resizable:false,
    mode:WindowMode::Windowed, 
   ..Default::default()
 }
}
fn setup_print(){
    println!("hello")
}
pub fn settings(){
   window();
   setup_print();
}
