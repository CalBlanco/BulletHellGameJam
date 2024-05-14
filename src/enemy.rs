use std::f32::consts::PI;

use bevy::{audio::Volume, prelude::*};
use rand::Rng;

use crate::{bullet::{self, BulletBundle}, gun, health, B_BOUND, PLAYBACK_SPEED, PLAYBACK_VOL};

use super::T_BOUND;

const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;



// Paths
const MELEE_PATH: EnemyPath = EnemyPath(|_| 0., |_| 1.5 );
const LINEAR_PATH: EnemyPath = EnemyPath(|_| 2. , |_| 2. );
const SPAMMER_PATH: EnemyPath = EnemyPath(|_| 0.75, |y| y.cos() + 0.2 );
const WAVY_PATH: EnemyPath = EnemyPath(|_| 0.5, |y| y.cos() + 0.1 );
const SPAWNER_PATH: EnemyPath = EnemyPath(|_| 0.1, |y| (3.0*y).cos() );

// Shot delays
const LINEAR_DELAY: (f32, f32) = (0.5, 5.5);
const SPAMMER_DELAY: (f32, f32) = (0.5, 1.5);
const SPAWNER_DELAY: (f32, f32) = (5.5, 30.5);

// GUN + BULLET BLUEPRINTS   
const GUN_BLUEPRINT_LINEAR: gun::GunBluePrint = gun::GunBluePrint(1.25, 20, 1);
const GUN_BLUEPRINT_WAVY: gun::GunBluePrint = gun::GunBluePrint(1.5, 100, 2);
const GUN_BLUEPRINT_SPAMMER: gun::GunBluePrint = gun::GunBluePrint(0.75, 10, 4);

const BULLET_STRAIGHT: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 5., |_| 0., 0., false, 20);
const BULLET_COS_POS: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 2., |_| 0., 0., false, 20);
const BULLET_COS_NEG: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 2., |_| 0., 0., false, 20);
const BULLET_DAIG_POS_0: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 8., |_| 4., 0., false, 20);
const BULLET_DAIG_POS_1: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 4., |_| 8., 0., false, 20);
const BULLET_DAIG_NEG_0: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 8., |_| -4., 0., false, 20);
const BULLET_DAIG_NEG_1: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 4., |_| -8., 0., false, 20);

const DEFAULT_FALL_SPEED: f32 = 20.;



#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;



#[derive(Component, Copy, Clone)]
/// Enemy type enum to determine movement / combat patterns
pub enum EnemyType {
    Melee, // Chase the player attempt to kamakazi them
    Linear, // shoot a straight shot either directly infront or at the player (maybe even fucking random)
    Wavy, // Shoot some cos/sin variant shot 
    Spammer, // shoot massive amounts of shit 
    Spawner, // Shoot some bursts but primarily spawn more of the other types of enemies when killed spawn 2 spawners lol (consequences)
}

impl EnemyType {
    pub fn get_score(&self) -> (u64, u64){
        match *self {
            EnemyType::Spawner => {(500, 3)},
            EnemyType::Spammer => {(400, 2)},
            EnemyType::Wavy => {(300, 1)},
            _ => {(50, 0)}
        }
    }
}

/// Container to make paths easier (shoulda done this for bullets too honestly)
pub struct EnemyPath(fn(f32)->f32, fn(f32)->f32);

#[derive(Component)]
pub struct Enemy {
    t: EnemyType,
    tick: f32,
    last_shot: f32,
    x_path: fn(f32) -> f32,
    y_path: fn(f32) -> f32,
    shot_range: (f32, f32),
    pub gun: gun::Gun
}

impl Enemy{
    pub fn get_type(&self) -> EnemyType {
        self.t
    }
}


#[derive(Bundle)]
pub struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    pub enemy: Enemy,
    collider: Collider,
    health: health::Health
}

/// Create a new enemey providing a spawn location, type and asset to render
impl EnemyBundle {
    pub fn new(spawn_x: f32, spawn_y: f32, t: EnemyType, asset: Handle<Image>, health: health::Health, path: EnemyPath, shot_range: (f32,f32), gun: gun::Gun) -> EnemyBundle{
        EnemyBundle {
            sprite_bundle: SpriteBundle {
                texture: asset,
                transform: Transform {
                    translation: Vec3::new(spawn_x, spawn_y, 0.),
                    rotation: Quat::from_rotation_z(PI),
                    scale: Vec3::new(1.,1.,1.)
                },
                ..default()
            },
            enemy: Enemy {
                tick: 0.,
                t: t,
                last_shot: 0.,
                x_path: path.0,
                y_path: path.1,
                shot_range: shot_range,
                gun: gun

            },
            collider: Collider,
            health: health
        }
    }



    
}



/// Control enemy movement and behavior 
pub fn enemy_control(
    time: Res<Time>,
    mut sprite_position: Query<(Entity, &mut Transform, &mut Enemy), With<Enemy>>,
    mut commands: Commands,
    mut asset_server: Res<AssetServer>

) {
    
    for(_, mut transform, mut enemy) in &mut sprite_position{
        enemy.tick += time.delta_seconds();
        // Implement bounding
        if transform.translation.y > T_BOUND as f32 { transform.translation.y -= DEFAULT_FALL_SPEED; continue;} // If the enemy is above the screen bounds we want it to drop down to the screen 
        if transform.translation.x < 0. - L_BOUND as f32 { transform.translation.x = R_BOUND as f32 - 1.; continue; } // Check to make sure we havent moved over the bounds ( if we have pacman across to the other side and continue moving)
        if transform.translation.x > R_BOUND as f32 { transform.translation.x = 0. - L_BOUND as f32 + 1.; continue; } 
        if transform.translation.y < B_BOUND { 
            transform.translation.y = T_BOUND as f32 + 50.;
            spawn_wave_box(2, &mut asset_server, &mut commands); // Consequence of letting an enemy get to the bottom
        }

        transform.translation.y += (enemy.y_path)(enemy.tick) * -1. as f32; // run the y function
        transform.translation.x += (enemy.x_path)(enemy.tick) * -1. as f32; // run the x function
        
        // Shot Logic (I wanna change this so they fire individually more often)
        enemy.last_shot += time.delta_seconds();
        let random_shot_delay: f32 = rand::thread_rng().gen_range(enemy.shot_range.0 .. enemy.shot_range.1);

        if enemy.last_shot > random_shot_delay && transform.translation.y < T_BOUND as f32{
            enemy.last_shot = 0. - random_shot_delay as f32;
            let spawn_x = transform.translation.x;
            let spawn_y = transform.translation.y - 30.;
            match enemy.t {
                EnemyType::Spawner => {
                    
                    let rng_size: u32 = rand::thread_rng().gen_range(2..20);
                    spawn_wave_box(rng_size, &mut asset_server, &mut commands);
                    enemy.last_shot -= 200.; // Set the spawn timer to have a larger delay than the shoot timer
                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/shieldhit.wav"),
                        // auto-despawn the entity when playback finishes
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Despawn,
                            volume: Volume::new(PLAYBACK_VOL),
                            speed: PLAYBACK_SPEED,
                            ..default()
                        },
                    });
                },
                EnemyType::Melee => {},
                _ => {
                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/laser.wav"),
                        // auto-despawn the entity when playback finishes
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Despawn,
                            volume: Volume::new(PLAYBACK_VOL),
                            speed: PLAYBACK_SPEED,
                            ..default()
                        },
                    });

                    let bullets = enemy.gun.get_bullets();
                    for bul in bullets {
                        commands.spawn(bullet::BulletBundle::new(spawn_x, spawn_y, bullet::Bullet::new(bul.0, bul.1, bul.2, bul.3, bul.4, bul.5),asset_server.load("plasma_red.png") ));
                    }
                }
            }

        }
        
    }
}


fn spawn_wave_box(wave_size: u32, asset_server: &mut Res<AssetServer>, commands: &mut Commands) {
    for _ in 1..wave_size { // spawns offset by 1
        
        
        let spawn_x = rand::thread_rng().gen_range( (0. - L_BOUND as f32)..(R_BOUND as f32));
        let spawn_y = rand::thread_rng().gen_range( (T_BOUND as f32)..(T_BOUND as f32 + 200.));

        let rng = rand::thread_rng().gen_range(0..=100);
        match rng {
            0..=25 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Melee, asset_server.load("enemies/melee.png"), health::Health::new(20,150, 3.5,5), MELEE_PATH, LINEAR_DELAY, gun::Gun::new(Vec::new(), 0., 0, 1)));},
            26..=50 => {
                let mut starting_bullets = Vec::new();
                starting_bullets.push(BULLET_STRAIGHT);
                commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Linear, asset_server.load("enemies/basic.png"), health::Health::new(0,150, 0.0, 5), LINEAR_PATH, LINEAR_DELAY, gun::Gun::new_from_blueprint(starting_bullets, GUN_BLUEPRINT_LINEAR)));
            },
            51..=75 => {
                let mut starting_bullets = Vec::new();
                starting_bullets.push(BULLET_COS_POS);
                starting_bullets.push(BULLET_COS_NEG);
                commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Wavy, asset_server.load("enemies/wavy.png"), health::Health::new(150,150, 3.0, 5), WAVY_PATH, LINEAR_DELAY, gun::Gun::new_from_blueprint(starting_bullets, GUN_BLUEPRINT_WAVY)));
            },
            76..=100 => {
                let mut starting_bullets = Vec::new();
                starting_bullets.push(BULLET_DAIG_NEG_0);
                starting_bullets.push(BULLET_DAIG_NEG_1);
                starting_bullets.push(BULLET_DAIG_POS_0);
                starting_bullets.push(BULLET_DAIG_POS_1);
                commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Spammer, asset_server.load("enemies/spammer.png"), health::Health::new(100,150, 3.0, 5), SPAMMER_PATH, SPAMMER_DELAY, gun::Gun::new_from_blueprint(starting_bullets, GUN_BLUEPRINT_SPAMMER)));
            },
            _ => ()
        }

        
        
    }

    let rng = rand::thread_rng().gen_range(2..4);
    for _ in 1..rng {
        let r_x = rand::thread_rng().gen_range( (0. - L_BOUND as f32)..(R_BOUND as f32));
        let r_y = rand::thread_rng().gen_range( (T_BOUND as f32)..(T_BOUND as f32 + 200.));

        commands.spawn(EnemyBundle::new(r_x, r_y, EnemyType::Spawner, asset_server.load("enemies/spawner.png"), health::Health::new(200,250, 3.0, 5), SPAWNER_PATH, SPAWNER_DELAY, gun::Gun::new(Vec::new(), 0., 0, 1)));
    }

}



pub fn init_wave(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>
){
        spawn_wave_box(20, &mut asset_server, &mut commands)
}
