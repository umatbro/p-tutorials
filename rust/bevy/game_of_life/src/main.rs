mod ui;
mod input;
mod simulations;

use bevy::prelude::*;
use crate::input::InputPlugin;
use crate::ui::MainMenuPlugin;
use crate::simulations::SimulationPlugin;

const GRID_SIZE: (i32, i32) = (100, 100);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1240.0,
            height: 720.0,
            title: "Game of life".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SimulationPlugin)
        .run();
}
