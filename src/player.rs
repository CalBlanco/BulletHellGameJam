use bevy::prelude::*;
use std::time::Duration;


use crate::{bullet, GameState};



const JUMP_SIZE: f32 = 400.;
const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;
const SPAWN_X: f32 = 0.;
const SPAWN_Y: f32 = -200.;
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
        println!("Dealt {} damage!", damage);
        self.shield = if self.shield - damage > 0 {self.shield - damage} else {0};
        self.health = if self.shield <= 0 && self.health - damage > 0 {self.health - damage} else {0};

        self.is_alive = if self.health > 0 {true} else {false};
    }

    // check if this entity is a live
    pub fn is_alive(&self) -> bool {
        println!("is dead: {}", self.is_alive);
        self.is_alive
    }

    pub fn get_health(&self) -> i64 {self.health}
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

        
        if transform.translation.x > R_BOUND as f32 {
            transform.translation.x -= 50.;
        } else if transform.translation.x < -(L_BOUND as f32) {
            transform.translation.x += 50.;
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
    
        if keycode.just_pressed(KeyCode::KeyW) {
            let jump_x = if transform.translation.x - JUMP_SIZE < 0. - L_BOUND as f32 { 0. - L_BOUND as f32 } else {transform.translation.x - JUMP_SIZE};
            transform.translation.x = jump_x;
        }
        if keycode.just_pressed(KeyCode::KeyS) {
            let jump_x = if transform.translation.x + JUMP_SIZE > L_BOUND as f32 { L_BOUND as f32 } else {transform.translation.x + JUMP_SIZE};
            transform.translation.x = jump_x;
        }
    
       
        // Shoot 
        if keycode.pressed(KeyCode::Space) && shot_timer.0.finished() {
            shot_timer.0.reset();
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/laser_0.wav"),
                // auto-despawn the entity when playback finishes
                settings: PlaybackSettings::DESPAWN,
            });
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| 10.*(a).cos()  ,  0.,  true, 20), asset_server.load("plasma_blue.png")));
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| -10.*(a).cos()  ,  0.,  true, 20), asset_server.load("plasma_blue.png")));
            commands.spawn( bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new(1, |_| 50., |_| 0. ,  0., true, 50), asset_server.load("plasma_blue.png")));
        }
    }
    else{
        game_state.set(GameState::Menu);
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

}