use glfw::{Context};
pub struct Window{
    pub glfw_window : glfw::Window,
    pub events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}
impl Window{
    
    pub fn new(
        win: glfw::Window,
        evt: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    ) -> Window {
        Window {
            glfw_window: win,
            events: evt,
        }
    }


}
pub fn create_window(width: u32, height: u32, name: String) -> (glfw::Glfw, Window) {
    let mut glfwer = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfwer.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfwer.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

      glfwer.window_hint(glfw::WindowHint::Samples(Some(1)));

    let (mut window, events) = glfwer
        .create_window(width, height, &name, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    //Yes i shadowed the old windows with this one :)
    window.set_key_polling(true);
    let window = Window::new(window, events);
    
    (glfwer, window)
}
pub fn make_context_current(window: &mut Window) {
    window.glfw_window.make_current();
//    window.glfw_window.set_all_polling(true);
    gl::load_with(|s| window.glfw_window.get_proc_address(s) as *const _);
}
