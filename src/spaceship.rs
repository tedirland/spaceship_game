use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROLL_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 25.0;

#[derive(Component, Debug)]
pub struct SpaceShip;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        SpaceShip,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<SpaceShip>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    // Define variables to managee movement and rotation
    let mut rotation: f32 = 0.0;
    let mut roll: f32 = 0.0;
    let mut movement: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED
    }
    // update the spaceship's velocity based on new direction;
    // check out blender 3d modeling software
    velocity.value = -transform.forward() * movement;
}
