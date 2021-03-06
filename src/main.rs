mod gl_common;
mod obj_loader;
//mod shape;
mod ecs;
mod tess_manager;
mod consts;

#[macro_use]
extern crate lazy_static;

use crate::ecs::components::follow_camera::FollowCamera;
use crate::ecs::components::physics::Physics;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::pulsate::Pulsate;
use crate::ecs::components::transformable::Transformable;
use crate::ecs::components::gun::Gun;
use crate::ecs::dispatcher::DoemDispatcher;
use crate::ecs::world::DoemWorld;
use clap::{App, Arg};
use doem_math::vector_space::{Matrix4, Vector3};
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};
use specs::prelude::*;
use specs::WorldExt;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let matches = App::new("Rusty obj viewer")
        .version("1.0")
        .author("Bram-Boris Meerlo and Peter-Jan Gootzen")
        .about("Made using our own linear algebra crate rusty_linear_algebra.")
        .arg(
            Arg::with_name("model_path")
                .short("m")
                .long("model")
                .value_name("MODEL_PATH")
                .help("Sets the wavefront obj model that is going to be loaded")
                .index(1)
                .required(true)
                .default_value(consts::DONUT_OBJ_PATH)
                .takes_value(true),
        )
        .get_matches();

    let model_path_str = matches.value_of("model_path").unwrap();

    start(&model_path_str);
}

fn start(model_path: &str) {
    let surface = GlfwSurface::new(WindowDim::Windowed(1600, 900), "Doem", WindowOpt::default())
        .expect("GLFW surface creation");


    let should_quit = Arc::new(Mutex::new(false));
    let mut world = DoemWorld::new();

    world
        .create_entity()
        .with(Shape::Unit {
            obj_path: model_path.to_owned(),
        })
        .with(Transform {
            position: Vector3::new_from_array([[0.0], [0.0], [0.0]]),
            scale: Vector3::new_from_array([[1.0], [1.0], [1.0]]),
            orientation: Matrix4::identity(),
        })
        .with(Physics {
            velocity: Vector3::new_from_array([[0.30], [0.0], [0.0]]),
        })
        .with(Transformable)
        .with(FollowCamera {
            zoom_level: 10.0,
            offset: Vector3::new_from_array([[20.0], [10.0], [0.0]]),
        })
//        .with(Pulsate {
//            speed: Vector3::new_from_array([[0.01], [0.01], [0.01]]),
//            current_direction: true,
//            min_scale: Vector3::new_from_array([[-5.0], [-5.0], [-5.0]]),
//            max_scale: Vector3::new_from_array([[5.0], [5.0], [5.0]]),
//        })
        .with(Gun {
            damage: consts::STARSHIP_BULLET_DAMAGE,
            velocity: consts::STARSHIP_BULLET_VELOCITY.clone()
        })
        .build();

    world
        .create_entity()
        .with(Shape::Unit {
            obj_path: consts::REFERENCEPLANE_OBJ_PATH.to_owned(),
        })
        .with(Transform {
            position: Vector3::new_from_array([[1.0], [-3.0], [0.0]]),
            scale: Vector3::new_from_array([[10.0], [1.0], [10.0]]),
            orientation: Matrix4::identity(),
        })
        .build();

    let mut dispatcher = DoemDispatcher::new(surface, should_quit.clone());
    dispatcher.setup(&mut world);
    'game_loop: loop {
        dispatcher.dispatch(&mut world);
        world.maintain();
        if *(*should_quit).lock().unwrap() {
            break 'game_loop;
        }
    }
}
