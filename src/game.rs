use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*, time::Stopwatch};

use crate::{bullet, enemy, explosion, health, player, power_ups};
use super::GameState;


pub struct BulletHellElite;

#[derive(Resource)]
pub struct GameTimer(pub Stopwatch);

#[derive(Resource)]
pub struct ScoreBoard { score: u64, mul: u64 }

#[derive(Component)]
pub struct MovingBackground(f32);

const SCROLL_SPEED: f32 = 0.25;

impl ScoreBoard {

    pub fn add_score(&mut self, inc: u64) {
        self.score = self.score + (inc * self.mul);
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }

    pub fn add_mul(&mut self, inc: u64){
        self.mul = if  self.mul + inc < 17 {self.mul+inc} else {16};
    }

    pub fn set_mul(&mut self, set: u64){
        self.mul = set;
    }

    pub fn get_mul(&self) -> u64 { self.mul }
}

impl Plugin for BulletHellElite {
    fn build(&self, app: &mut App){
        app
            .add_event::<bullet::CollisionEvent>()
            .add_event::<bullet::ScoreEvent>()
            .insert_resource(ScoreBoard {score: 0, mul: 1})
            .add_systems(OnEnter(GameState::Game),(setup, player::spawn_player, enemy::init_wave, explosion::setup).before(player::sprite_movement))
            .add_systems(FixedPreUpdate, advance_game_timer.run_if(in_state(GameState::Game)))
            .add_systems(FixedUpdate, (
                player::sprite_movement, 
                bullet::bullet_movement, 
                bullet::apply_collision_damage, 
                bullet::update_score, 
                bullet::bullet_on_bullet_collision,
                enemy::enemy_control, 
                enemy::wave_manager,
                player::update_player_score, 
                player::update_time_display,
                player::update_health_display,
                player::update_ammo_display,
                health::shield_tick, 
                power_ups::move_powerups,
                power_ups::handle_powerup_collision,
                move_background_image
                ).run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

/// Setup our game world 
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    

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

    commands.spawn(
        (
            SpriteBundle {
            texture: asset_server.load("bckg.png"),
            transform: Transform {
                translation: Vec3::new(0.,0., -1.0),
                scale: Vec3::new(0.6, 0.6, 1.0),
                ..default()
            },
            ..default()
            },
            MovingBackground(1.0)
        ));

    commands.insert_resource(GameTimer(Stopwatch::new()));


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

pub fn move_background_image(
    mut bckg: Query<(&mut Transform, &mut MovingBackground), With<MovingBackground>>
){
    if let Ok((mut transform, mut back)) = bckg.get_single_mut() {
        if transform.translation.y < -400. || transform.translation.y > 300. {back.0 = back.0 * -1.0}
        transform.translation.y += SCROLL_SPEED * back.0;
        
    }
}

pub fn advance_game_timer(time: Res<Time>, mut game_timer: ResMut<GameTimer>){
    game_timer.0.tick(time.delta());
}

