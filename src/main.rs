use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use player::PlayerPlugin;
use exp_orb::ExpOrbPlugin;
use ui::GameUI;

mod exp_orb;
mod player;
mod ui;

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin { 
        min_width: 256.0, min_height: 144.0 
    };

    commands.spawn(camera);
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
            WorldInspectorPlugin::default()
                .run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins((PlayerPlugin, ExpOrbPlugin, GameUI))
        .add_systems(Startup, setup)
        .run()
}