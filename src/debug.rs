use bevy::prelude::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_transform);
    }
}

fn print_transform(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at transform {:?}.", entity, transform)
    }
}
