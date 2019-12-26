
use glfw::{Context};
mod io;
mod graphics;
mod components;
mod helpers;
use specs::{World, WorldExt, Builder,RunNow};
use nalgebra as na;
fn main() {
   let mut engine = helpers::quickinit::Engine::new();

    engine.init_basic_systems();
    let vertshdr = graphics::shader::shader_from_source(
        &std::ffi::CString::new(include_str!("sprite.vert")).unwrap(),
        gl::VERTEX_SHADER,
    ).unwrap();
    let fragshdr = graphics::shader::shader_from_source(
        &std::ffi::CString::new(include_str!("sprite.frag")).unwrap(),
        gl::FRAGMENT_SHADER,
    ).unwrap();
    let program_id = graphics::shader::create_program(vertshdr, fragshdr);
    unsafe {
        gl::UseProgram(program_id);
        
    }
    engine.world.create_entity().with(components::Transform{position : na::Vector2::<f32>::new(100.0,100.0),scale : na::Vector2::<f32>::new(100.0,100.0),rotation : 0.0}).with(components::Sprite{texid : 0}).build();
    engine.update(|dt|{},|dt|{});

    println!("Hello, world!");
}
