use bevy::prelude::*;

use crate::exp_orb::Experience;

pub struct GameUI;

#[derive(Component)]
pub struct ExpText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_exp_ui);
    }
}

fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("UI Root,"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Experience!",
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                },
                ExpText,
            ));
        });
}

fn update_exp_ui(mut texts: Query<&mut Text, With<ExpText>>, experience: Res<Experience>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Experience:  {:?}", experience.0);
    }
}
