use specs::{ WorldExt, RunNow};
use super::super::graphics::*;
use super::super::io;
use super::super::components;
use glfw::{Context};
use std::rc::Rc;
use std::cell::RefCell;
pub struct Engine{
    pub world: specs::World,
    render_commands_system : render_system::RenderCommandsSystem,
    window_handler : (glfw::Glfw,io::window::Window)

}
impl Engine{
    pub fn new() -> Self{
        let  (glfwer,mut window) = io::window::create_window(1024,720,"PhiEngine".to_owned());
        io::window::make_context_current(&mut window);
        let mut render = renderer::Renderer::new();
        render.init();
        let mut render_system = render_system::RenderSystem::new(render);
        render_system.init();
        let render_commands_system = render_system::RenderCommandsSystem{m_renderer : Rc::new(RefCell::new(render_system))};
        Engine{
            world : specs::World::new(),        
            render_commands_system,
            window_handler : (glfwer,window)
        }
    }
    pub fn init_basic_systems(&mut self){
        self.world.register::<components::Sprite>();
        self.world.register::<components::Transform>();
    }
    pub fn update<U:Fn(f32),FU: Fn(f32)>(&mut self,update : U,fixed_update : FU){
        let dt = 0.01;
       
        let mut t = 0.0;
        let mut current_time = std::time::Instant::now();
        let mut accumulator:f64 = 0.0;



        while !self.window_handler.1.glfw_window.should_close(){
            let new_time = std::time::Instant::now();
            let mut frame_time = new_time.duration_since(current_time).as_nanos() as f32 / 1000000000.0; // from ns to s
            if frame_time > 0.25 { // where did this constant come from?
                frame_time = 0.25;
            }
            current_time = new_time;

            accumulator += frame_time as f64;

            while accumulator >= dt {
                accumulator -= dt;
                fixed_update(frame_time);
                t += dt;
            }

            self.window_handler.0.poll_events();
            unsafe{
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::ClearColor(1.0,0.0,0.0,1.0);
            }
            update(frame_time);
            self.render_commands_system.run_now(&self.world);

            self.window_handler.1.glfw_window.swap_buffers();
        }
    }
}
