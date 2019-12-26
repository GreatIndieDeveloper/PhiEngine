//This is jsut the components collection that will be used for ECS i put it here for lack of a better place
use specs::{Component, VecStorage};
use nalgebra as na;

pub struct Transform{
    pub position : na::Vector2<f32>,
    pub rotation : f32,
    pub scale : na::Vector2<f32>
}
impl Component for Transform {
    type Storage = VecStorage<Self>;
}


pub struct Sprite{
    pub texid : gl::types::GLuint
}
impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

