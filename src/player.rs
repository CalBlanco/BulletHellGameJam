use bevy::{audio::Volume, prelude::*};
use std::time::Duration;


use crate::{bullet, game::{self, ScoreBoard}, GameState, PLAYBACK_SPEED, PLAYBACK_VOL};

use super::{EzTextBundle, B_BOUND};



const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;
const PLAYER_T_BOUND: f32 = -200.;
const SPAWN_X: f32 = 0.;
const SPAWN_Y: f32 = B_BOUND + 100.;

const MOVE_SPEED: f32 = 180.;
const SHOT_DELAY: f32 = 0.05;


const SHIELD_SIZE: i64 = 32_500;
const HEALTH_SIZE: i64 = 32_500;

const BULLET_DAMAGE: i64 = 20;


#[derive(Resource)]
pub struct ShotTimer(Timer);

#[derive(Resource)]
pub struct ShieldTimer(Timer);


// TODO: implement a shield reset timer

#[derive(Component)]
/// Player information / tag
pub struct PlayerControlled;

pub struct BulletBlueprint(pub i8, pub fn(f32)->f32, pub fn(f32)->f32, pub f32, pub bool, pub i64);

#[derive(Component)]
pub struct Gun {
    bullet_blueprints: Vec<BulletBlueprint>,
    damage: i64,
    shoot_delay: f32,
    max_bullets: u8,
    pub shot_timer: ShotTimer
}

impl Gun {
    pub fn new(starting_bullets: Vec<BulletBlueprint>, init_shoot_delay: f32, init_damage: i64, max_bullets: u8) -> Gun{    
        Gun {
            bullet_blueprints: if starting_bullets.is_empty() { Vec::new()} else {starting_bullets},
            damage: init_damage,
            shoot_delay: init_shoot_delay,
            max_bullets: max_bullets,
            shot_timer: ShotTimer(Timer::from_seconds(init_shoot_delay, TimerMode::Repeating))
        }
    }

    pub fn add_bullet(&mut self, blueprint: BulletBlueprint){
        if self.bullet_blueprints.len() < self.max_bullets as usize {
            self.bullet_blueprints.push(blueprint)
        }
    }

    fn get_bullets(&self) -> &Vec<BulletBlueprint> { &self.bullet_blueprints }

    pub fn set_bullet_delay(&mut self, new_delay: f32) {

        self.shoot_delay = if  new_delay > 0. {new_delay} else {0.01};
        self.shot_timer.0.set_duration(Duration::from_secs_f32(self.shoot_delay));
    }

    pub fn get_bullet_delay(&self) -> f32 { self.shoot_delay }

    pub fn set_bullet_damage(&mut self, new_damage: i64) {
        self.damage = new_damage;
    }

    pub fn get_bullet_damage(&self) -> i64{
        self.damage
    }

    pub fn tick_time(&mut self, time: Duration){
        self.shot_timer.0.tick(time);
    }

    pub fn can_shoot(&self) -> bool {
        self.shot_timer.0.finished()
    }

    pub fn reset_shot_timer(&mut self){
        self.shot_timer.0.reset();
    }
}


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
    pub timer: ShieldTimer
}

impl Health {
    /// Create a new health component specifying shield size, and health
    pub fn new(shield_size: i64, health_size: i64, shield_time: f32) -> Health {
        Health {
            shield: shield_size,
            health: health_size,
            is_alive: true,
            timer: ShieldTimer(Timer::new(Duration::from_secs_f32(shield_time), TimerMode::Repeating)) 
        }
    }

    /// do damage to the entity
    pub fn damage(&mut self, damage: i64){
        self.timer.0.reset();

        if self.shield > 0 { // shield is up
            self.shield = self.shield - damage;
        }
        else { // shields not up
            self.health = self.health - damage;
        }
        
        self.shield = if self.shield < 0 {0} else {self.shield};

        self.is_alive = self.health > 0 
    }

    /// check if this entity is a live
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn get_health(&self) -> i64 {self.health}
    pub fn get_shield(&self) -> i64 {self.shield}

    pub fn regen_shield(&mut self, inc: i64){self.shield = self.shield + inc;}
}

pub fn sprite_movement(
    time: Res<Time>, 
    mut sprite_position: Query<(Entity, &mut Transform, &mut Gun), With<PlayerControlled>>,
    keycode: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut shot_timer: ResMut<ShotTimer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    
    if let Ok((_, mut transform , mut gun)) = sprite_position.get_single_mut() {
        gun.tick_time(time.delta());

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
        if keycode.pressed(KeyCode::Space) && gun.can_shoot() {
            gun.reset_shot_timer();
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/laser_0.wav"),
                // auto-despawn the entity when playback finishes
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: Volume::new(PLAYBACK_VOL),
                    speed: PLAYBACK_SPEED,
                    ..default()
                },
            });

            

            let bullets = gun.get_bullets();

            for bul in bullets {
                commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new(bul.0, bul.1, bul.2, bul.3, bul.4, bul.5), asset_server.load("plasma_blue.png")));
            }

            /* commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| 5.*(a).cos()  ,  0.,  true, bullet_damage), asset_server.load("plasma_blue.png")));
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| -5.*(a).cos()  ,  0.,  true, bullet_damage), asset_server.load("plasma_blue.png")));
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 30., |_| 0.  ,  0.,  true, bullet_damage), asset_server.load("plasma_green.png"))); */
        }
    }
    else{
        game_state.set(GameState::Menu); // restart game when player is unfindable (dead)
    }
}


#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    control: PlayerControlled,
    health: Health,
    gun: Gun
}

impl PlayerBundle {
    fn new(asset: Handle<Image>) -> PlayerBundle {
        let mut starting_bullets = Vec::new();
        starting_bullets.push(BulletBlueprint(1,|_| 20., |x| x.cos(), 0., true, 50));
        starting_bullets.push(BulletBlueprint(1,|_| 20., |x| -x.cos(), 0., true, 50));
        starting_bullets.push(BulletBlueprint(1,|_| 20., |_: f32| 0., 0., true, 50));
        starting_bullets.push(BulletBlueprint(1,|_| 2., |x| 10.*x.cos(), 0., true, 50));
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset,
                transform: Transform::from_xyz(SPAWN_X, SPAWN_Y, 1.),
                ..default()
            },
            control: PlayerControlled,
            health: Health::new(SHIELD_SIZE, HEALTH_SIZE, 2.0),
            gun: Gun::new(starting_bullets, SHOT_DELAY, BULLET_DAMAGE, 20),
        }
    } 
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    
    let asset = asset_server.load("player.png");
    
    commands.spawn(
        PlayerBundle::new(asset)
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


pub fn shield_tick(
    time: Res<Time>,
    mut query: Query<&mut Health>
){
    for mut health in query.iter_mut(){
        health.timer.0.tick(time.delta()); // increment timer

        if health.timer.0.finished() {
            health.regen_shield(3);
        }

    }
}