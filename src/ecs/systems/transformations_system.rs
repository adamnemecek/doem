use crate::ecs::components::shape::Shape;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::transformable::Transformable;
use crate::ecs::resources::doem_events::DoemEvents;
use doem_math::vector_space::{Matrix4, Scalar, Vector4};
use luminance_glfw::{Action, Key, WindowEvent};
use specs::prelude::*;

pub struct TransformationsSystem;

impl<'a> System<'a> for TransformationsSystem {
    type SystemData = (
        Read<'a, DoemEvents>,
        ReadStorage<'a, Transformable>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Shape>,
    );

    fn run(&mut self, (events, transformable, mut transform, shape): Self::SystemData) {
        for (_t, t, s) in (&transformable, &mut transform, &shape).join() {
            for event in &events.0 {
                match event {
                    WindowEvent::Key(Key::A, _, action, _)
                    | WindowEvent::Key(Key::Left, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.position[2][0] += 0.010;
                    }

                    WindowEvent::Key(Key::D, _, action, _)
                    | WindowEvent::Key(Key::Right, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.position[2][0] += -0.010;
                    }

                    WindowEvent::Key(Key::W, _, action, _)
                    | WindowEvent::Key(Key::Up, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.position.data[0][0] += -0.010;
                    }

                    WindowEvent::Key(Key::S, _, action, _)
                    | WindowEvent::Key(Key::Down, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.position.data[0][0] += 0.010;
                    }

                    WindowEvent::Key(Key::K, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.scale.data[0][0] += 0.1;
                        t.scale.data[1][0] += 0.1;
                        t.scale.data[2][0] += 0.1;
                    }
                    WindowEvent::Key(Key::J, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.scale.data[0][0] -= 0.1;
                        t.scale.data[1][0] -= 0.1;
                        t.scale.data[2][0] -= 0.1;
                    }
                    WindowEvent::Key(Key::Q, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.orientation = Self::rotate_y(&t.orientation, s, 0.05);
                    }
                    WindowEvent::Key(Key::E, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.orientation = Self::rotate_y(&t.orientation, s, -0.05);
                    }
                    WindowEvent::Key(Key::Z, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.orientation = Self::rotate_x(&t.orientation, s, 0.05);
                    }
                    WindowEvent::Key(Key::X, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.orientation = Self::rotate_x(&t.orientation, s, -0.05);
                    }
                    WindowEvent::Key(Key::T, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.orientation = Self::rotate_z(&t.orientation, s, 0.05);
                    }
                    WindowEvent::Key(Key::G, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        t.orientation = Self::rotate_z(&t.orientation, s, -0.05);
                    }

                    _ => (),
                }
            }
        }
    }
}

impl TransformationsSystem {
    pub fn rotate_x(orientation: &Matrix4, shape: &Shape, angle: Scalar) -> Matrix4 {
        if let Shape::Init { bounding_box, .. } = shape {
            println!("Rotating X with: {}rad", angle);
            let x = Vector4::new_from_array([
                [bounding_box.middle_point[0][0] + bounding_box.half_size[0][0]],
                [0.0],
                [0.0],
                [0.0],
            ]);
            let x_norm = x.normalize();
            return orientation * &Matrix4::get_rotation_matrix(&x_norm, angle);
        }
        Matrix4::identity()
    }
    pub fn rotate_y(orientation: &Matrix4, shape: &Shape, angle: Scalar) -> Matrix4 {
        if let Shape::Init { bounding_box, .. } = shape {
            println!("Rotating Y with: {}rad", angle);
            let y = Vector4::new_from_array([
                [0.0],
                [bounding_box.middle_point[1][0] + bounding_box.half_size[1][0]],
                [0.0],
                [0.0],
            ]);
            let y_norm = y.normalize();
            return orientation * &Matrix4::get_rotation_matrix(&y_norm, angle);
        }
        Matrix4::identity()
    }
    pub fn rotate_z(orientation: &Matrix4, shape: &Shape, angle: Scalar) -> Matrix4 {
        if let Shape::Init { bounding_box, .. } = shape {
            println!("Rotating Z with: {}rad", angle);
            let z = Vector4::new_from_array([
                [0.0],
                [0.0],
                [bounding_box.middle_point[2][0] + bounding_box.half_size[2][0]],
                [0.0],
            ]);
            let z_norm = z.normalize();
            return orientation * &Matrix4::get_rotation_matrix(&z_norm, angle);
        }
        Matrix4::identity()
    }
}
