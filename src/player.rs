use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::exp_orb::Experience;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement,))
            .insert_resource(Experience(0))
            .register_type::<Player>();
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
    pub health: f32,
}

fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_speed = player.speed;

        // Calculate the player movement vector by taking user input
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        // Normalize the vector so that diagonal movement isn't faster
        let direction = direction.normalize_or_zero();

        transform.translation += direction * movement_speed * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("sprites_defacto/Jake.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player {
            speed: 100.0,
            health: 100.0,
        },
        Name::new("Player"),
    ));
}
