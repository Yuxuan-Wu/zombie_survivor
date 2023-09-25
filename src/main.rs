use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use exp_orb::ExpOrbPlugin;
use player::*;
use ui::GameUI;

mod exp_orb;
mod player;
mod ui;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 512.0,
        min_height: 288.0,
    };

    commands.spawn(camera).insert(MainCamera);
}

pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(Transform { translation: player_translation, .. }) = player_query.get_single() {
        let player_translation = *player_translation;

        // Get the 2D camera entity
        let mut main_camera = camera_query.get_single_mut().unwrap();

        // Update camera position to follow the player
        main_camera.translation.x = player_translation.x;
        main_camera.translation.y = player_translation.y;
    }
}


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Zombie Survivor".into(),
                        resolution: (1080.0, 810.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins((PlayerPlugin, ExpOrbPlugin, GameUI))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_follow_player)
        .run()
}
