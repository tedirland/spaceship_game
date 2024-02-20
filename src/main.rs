mod asset_loader;
mod asteroids;
mod camera;
mod collission_detection;
mod debug;
mod despawn;
mod movement;
mod schedule;
mod spaceship;
mod state;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collission_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;

fn main() {
    App::new()
        // Bevy buiilt-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        // User configured plugins
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .run();
}
