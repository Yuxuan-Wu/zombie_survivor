use bevy::prelude::*;

use crate::player::Player;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Experience(pub u32);

pub struct ExpOrbPlugin;

impl Plugin for ExpOrbPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_exporb_parent)
            .add_systems(Update, (spawn_exporb, exporb_lifetime))
            .register_type::<ExpOrb>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ExpOrb {
    pub exp: u32,
    pub lifetime: Timer,
}

fn spawn_exporb(
    mut commands: Commands,
    assest_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut exp: ResMut<Experience>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<ExpOrbParent>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    let texture: Handle<Image> = assest_server.load("sprites_defacto/ExpOrb.png");

    commands.entity(parent).with_children(|commands| {
        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            ExpOrb {
                exp: 10,
                lifetime: Timer::from_seconds(10.0, TimerMode::Once),
            },
            Name::new("Exp Orb"),
        ));
    });
}

fn exporb_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut exp: ResMut<Experience>,
    mut exp_orbs: Query<(Entity, &mut ExpOrb)>,
) {
    for (entity, mut exp_orb) in &mut exp_orbs {
        exp_orb.lifetime.tick(time.delta());

        if exp_orb.lifetime.finished() {
            exp.0 += exp_orb.exp;
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct ExpOrbParent;

fn spawn_exporb_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        ExpOrbParent,
        Name::new("Exp Orb Parent"),
    ));
}
