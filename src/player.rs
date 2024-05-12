use bevy::{audio::Volume, prelude::*};
use std::time::Duration;


use crate::{bullet, game, GameState};

use super::{EzTextBundle, B_BOUND};



const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;
const PLAYER_T_BOUND: f32 = -200.;
const SPAWN_X: f32 = 0.;
const SPAWN_Y: f32 = B_BOUND + 100.;

const MOVE_SPEED: f32 = 180.;
const SHOT_DELAY: f32 = 0.05;


const SHIELD_SIZE: i64 = 400;
const HEALTH_SIZE: i64 = 200;


#[derive(Resource)]
pub struct ShotTimer(Timer);



// TODO: implement a shield reset timer

#[derive(Component)]
/// Player information / tag
pub struct PlayerControlled;



#[derive(Component)]
pub struct HealthText;
#[derive(Component)]
pub struct ShieldText;
#[derive(Component)]
pub struct ScoreText;





#[derive(Component)]
pub struct Health {
    shield: i64,
    health: i64,
    is_alive: bool,
}

impl Health {
    /// Create a new health component specifying shield size, and health
    pub fn new(shield_size: i64, health_size: i64) -> Health {
        Health {
            shield: shield_size,
            health: health_size,
            is_alive: true,
            
        }
    }

    /// do damage to the entity
    pub fn damage(&mut self, damage: i64){
        if self.shield > 0 { // shield is up
            self.shield = self.shield - damage;
        }
        else { // shields not up
            self.health = self.health - damage;
        }
        

        self.is_alive = self.health > 0 
    }

    /// check if this entity is a live
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn get_health(&self) -> i64 {self.health}
    pub fn get_shield(&self) -> i64 {self.shield}
}

pub fn sprite_movement(
    time: Res<Time>, 
    mut sprite_position: Query<(Entity, &mut Transform), With<PlayerControlled>>,
    keycode: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut shot_timer: ResMut<ShotTimer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    
    if let Ok((_, mut transform )) = sprite_position.get_single_mut() {
        shot_timer.0.tick(time.delta());

        //gizmos.rect_2d(transform.translation.truncate(), 0., Vec2::new(32., 32.), Color::rgb(1.,1.,0.));

        // Bound X
        if transform.translation.x > R_BOUND as f32 {
            transform.translation.x  = (R_BOUND - 1) as f32;
        } else if transform.translation.x < -(L_BOUND as f32) {
            transform.translation.x = 1. - L_BOUND as f32 ;
        }
        // Bound Y 
        if transform.translation.y > PLAYER_T_BOUND as f32 {
            transform.translation.y  = (PLAYER_T_BOUND - 1.) as f32;
        } else if transform.translation.y < SPAWN_Y {
            transform.translation.y = SPAWN_Y + 1.; 
        }


    
        let speed_mult = if keycode.pressed(KeyCode::ShiftLeft){ 3. } else { 1.}; // Speed boost
        let move_dist = MOVE_SPEED * time.delta_seconds() * speed_mult;
        //Move Left
        if keycode.pressed(KeyCode::KeyA) || keycode.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= move_dist; // if transform.translation.x - move_dist < -500. {0.} else {move_dist}
        }
        // Move Right
        if keycode.pressed(KeyCode::KeyD) || keycode.pressed(KeyCode::ArrowRight) {
            transform.translation.x +=  move_dist;//if transform.translation.x + move_dist > R_BOUND as f32 {0.} else {move_dist}
        }
    
        if keycode.pressed(KeyCode::KeyW){ 
            transform.translation.y += move_dist;
        }
        if keycode.pressed(KeyCode::KeyS){
            transform.translation.y -= move_dist;
        }
    
       
        // Shoot 
        if keycode.pressed(KeyCode::Space) && shot_timer.0.finished() {
            shot_timer.0.reset();
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/laser_0.wav"),
                // auto-despawn the entity when playback finishes
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: Volume::new(0.25),
                    ..default()
                },
            });
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| 5.*(a).cos()  ,  0.,  true, 60), asset_server.load("plasma_blue.png")));
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| -5.*(a).cos()  ,  0.,  true, 60), asset_server.load("plasma_blue.png")));
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 30., |_| 0.  ,  0.,  true, 20), asset_server.load("plasma_green.png")));
        }
    }
    else{
        game_state.set(GameState::Menu); // restart game when player is unfindable (dead)
    }
}


pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    
    let asset = asset_server.load("player.png");
    
    commands.spawn((
        SpriteBundle {
            texture: asset,
            transform: Transform::from_xyz(SPAWN_X, SPAWN_Y, 1.),
            ..default()
        },
        PlayerControlled,
        Health::new(SHIELD_SIZE, HEALTH_SIZE)
    )
    );
    commands.insert_resource(ShotTimer(Timer::new(Duration::from_secs_f32(SHOT_DELAY), TimerMode::Repeating)));
 
    commands.spawn(EzTextBundle::new(String::from("Health: "), 60., 820., 20., asset_server.load("fonts/Lakmus.ttf"), Color::TEAL,HealthText));
    commands.spawn(EzTextBundle::new(String::from("Shield: "), 60., 880., 20., asset_server.load("fonts/Lakmus.ttf"), Color::TEAL,ShieldText));
    commands.spawn(EzTextBundle::new(String::from("Score: "), 80., 760., 20., asset_server.load("fonts/Lakmus.ttf"), Color::GOLD,ScoreText));

}

pub fn update_player_health(mut query: Query<&mut Text, With<HealthText>>, mut player: Query<&mut Health, With<PlayerControlled>>){
    for mut text in &mut query {
        if let Ok(health) = player.get_single_mut() {
            text.sections[0].value = format!("Health: {:.2}", health.get_health());
        }
    }
}

pub fn update_player_shield(mut query: Query<&mut Text, With<ShieldText>>, mut player: Query<&mut Health, With<PlayerControlled>>){
    for mut text in &mut query {
        if let Ok(health) = player.get_single_mut() {
            text.sections[0].value = format!("Shield: {:.2}", health.get_shield());
        }
    }
}

pub fn update_player_score(
    mut query: Query<&mut Text, With<ScoreText>>, 
    scoreboard: Res<game::ScoreBoard>
) {
    for mut text in &mut query {
        text.sections[0].value = format!("{:09} x {:01}", scoreboard.get_score(), scoreboard.get_mul())
    }
}