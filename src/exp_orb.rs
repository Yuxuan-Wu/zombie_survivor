use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Experience(pub u32);

pub struct ExpOrbPlugin;

impl Plugin for ExpOrbPlugin {
	fn build(&self, app: &mut App) {
		
	}
}