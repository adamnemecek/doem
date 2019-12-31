mod gl_common;
mod obj_loader;
//mod shape;
mod ecs;
mod tess_manager;

use crate::ecs::components::shape::Shape;

use clap::{Arg, App};
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};
use specs::WorldExt;
use specs::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use crate::ecs::world::DoemWorld;
use crate::ecs::dispatcher::DoemDispatcher;

fn main() {
    let matches = App::new("Rusty obj viewer")
                          .version("1.0")
                          .author("Bram-Boris Meerlo and Peter-Jan Gootzen")
                          .about("Made using our own linear algebra crate rusty_linear_algebra.")
                          .arg(Arg::with_name("model_path")
                               .short("m")
                               .long("model")
                               .value_name("MODEL_PATH")
                               .help("Sets the wavefront obj model that is going to be loaded")
                               .index(1)
                               .required(true)
                               .takes_value(true))
                          .get_matches();

    let model_path_str = matches.value_of("model_path").unwrap();

    start(&model_path_str);
}

fn start(model_path: &str) {
    let surface = GlfwSurface::new(
        WindowDim::Windowed(1600, 900),
        "Doem",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation");

    let should_quit = Arc::new(Mutex::new(false));
    let mut world = DoemWorld::new();

    world.create_entity().with(Shape::Unit { obj_path: model_path.to_owned() }).build();
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
