use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*};

use crate::{bullet, enemy, player};
use super::GameState;


pub struct BulletHellElite;

impl Plugin for BulletHellElite {
    fn build(&self, app: &mut App){
        app
            .add_event::<bullet::CollisionEvent>()
            .add_systems(OnEnter(GameState::Game),(setup, player::spawn_player, enemy::init_wave).before(player::sprite_movement))
            .add_systems(FixedUpdate, (player::sprite_movement, bullet::bullet_movement, bullet::play_collision_sound, enemy::enemy_control).run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

/// Setup our game world 
fn setup(mut commands: Commands) {
    

    commands.spawn(( //Camera with bloom settings enabled
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));


}

fn cleanup(
    ents: Query<(Entity, &Transform)>,
    cams: Query<Entity, With<Camera>>,
    mut commands: Commands
){
    for (ent, _) in &ents {
        commands.entity(ent).despawn();
    }

    for ent in &cams{
        commands.entity(ent).despawn();
    }
}