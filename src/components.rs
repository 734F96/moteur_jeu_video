use specs::
{
    Component,
    DenseVecStorage
};
use graphics::{Object, Handle, nalgebra::Vector3};


#[derive(Debug, Clone, Copy)]
pub struct Spatial
{
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Model(pub Handle<Object>);

impl Component for Spatial
{
    type Storage = DenseVecStorage<Self>;
}

impl Component for Model
{
    type Storage = DenseVecStorage<Self>;
}


use graphics::Light;


#[derive(Debug, Clone, Copy)]
pub struct Lighting(pub Light);

impl Component for Lighting
{
    type Storage = DenseVecStorage<Self>;
}
