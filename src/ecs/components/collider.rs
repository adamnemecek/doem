use doem_math::vector_space::Vector3;
use specs::prelude::*;

pub struct Collider {
    pub half_size: Vector3,
}

impl Component for Collider {
    type Storage = VecStorage<Self>;
}
