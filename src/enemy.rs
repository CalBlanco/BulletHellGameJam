use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use rand::Rng;

use crate::{bullet, game::GameTimer, gun, health, shapes::{self, generate_circle, generate_line, generate_square, generate_triangle}, B_BOUND};

use super::T_BOUND;

const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;



// Paths
const MELEE_PATH: EnemyPath = EnemyPath(|_| 0., |y| y*y / 30. );
const LINEAR_PATH: EnemyPath = EnemyPath(|_| 2. , |_| 0.5 );
const SPAMMER_PATH: EnemyPath = EnemyPath(|_| 0.75, |y| y.cos() + 0.2 );
const WAVY_PATH: EnemyPath = EnemyPath(|_| 0.5, |y| 3.*y.cos() + 0.1 );
const SPAWNER_PATH: EnemyPath = EnemyPath(|_| 0.1, |y| (3.0*y).cos() + (1./(10.*y)));

// Shot delays
const LINEAR_DELAY: (f32, f32) = (0.5, 2.5);
const SPAMMER_DELAY: (f32, f32) = (0.5, 1.5);
const SPAWNER_DELAY: (f32, f32) = (1.5, 7.5);

// GUN + BULLET BLUEPRINTS   
const GUN_BLUEPRINT_LINEAR: gun::GunBluePrint = gun::GunBluePrint(1.25, 20, 1, 1000, 2.0);
const GUN_BLUEPRINT_WAVY: gun::GunBluePrint = gun::GunBluePrint(1.5, 100, 2, 1000, 2.0);
const GUN_BLUEPRINT_SPAMMER: gun::GunBluePrint = gun::GunBluePrint(0.75, 10, 4, 3000, 2.0);

const BULLET_STRAIGHT: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 5., |_| 0., 0., false, 20);
const BULLET_COS_POS: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 2., |_| 0., 0., false, 20);
const BULLET_COS_NEG: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 2., |_| 0., 0., false, 20);
const BULLET_DAIG_POS_0: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 8., |_| 4., 0., false, 20);
const BULLET_DAIG_POS_1: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 4., |_| 8., 0., false, 20);
const BULLET_DAIG_NEG_0: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 8., |_| -4., 0., false, 20);
const BULLET_DAIG_NEG_1: gun::BulletBlueprint = gun::BulletBlueprint(-1, |_| 4., |_| -8., 0., false, 20);

const DEFAULT_FALL_SPEED: f32 = 20.;

// Wave constants
const WAVE_SIZE: u32 = 15; // multiplied by time elapsed (in minutes)
const WAVE_INTERVAL: f32 = 45.;  // divided by time elapsed (in minutes)



#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;


#[derive(Resource)]
pub struct WaveTimer(pub Timer);

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

const TICK_MAX: f32 = 60. * 3.; // reset tick after 3 min  

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
            enemy.tick = if enemy.tick > TICK_MAX { 0. } else {enemy.tick};
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
                    
                    let rng_rad: f32 = rand::thread_rng().gen_range(100. .. 580.);
                    let size = ((rng_rad / 100.) * 25. ) as usize;
                    let roll = rand::thread_rng().gen_range(0..3);
                    let x = transform.translation.x;
                    let y = transform.translation.y;
                    let points = match roll {
                        0 => {
                            generate_circle(x, y, rng_rad, size as usize)
                        },
                        1 => {
                            generate_square(x, y, rng_rad, size/2)
                        },
                        2 => {
                            
                            generate_triangle((x - rng_rad/2., y), (x + rng_rad/2., y), (x, y+rng_rad/2.), size / 3)
                        }
                        _ => {
                            generate_line(x - rng_rad/2., y - rng_rad/2., x + rng_rad/2., y + rng_rad/2., size)
                        }
                    };

                    
                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/shieldhit.wav"),
                        // auto-despawn the entity when playback finishes
                        settings: PlaybackSettings::DESPAWN,
                    });
                    for p in points {
                        commands.spawn(bullet::BulletBundle::new(p.0, p.1, bullet::Bullet::new(-1, |y| y*y, |_| 0., 0., false, 50),asset_server.load("plasma_purple.png") ));
                    }
                },
                EnemyType::Melee => {},
                _ => {
                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/laser.wav"),
                        // auto-despawn the entity when playback finishes
                        settings: PlaybackSettings::DESPAWN
                    });

                    let bullets = enemy.gun.get_bullets();
                    for bul in bullets {
                        commands.spawn(bullet::BulletBundle::new(spawn_x, spawn_y, bullet::Bullet::new(bul.0, bul.1, bul.2, bul.3, bul.4, bul.5),asset_server.load("plasma_red.png") ));
                    }
                }
            }

        }
        
    }

    if sprite_position.iter().len() == 0 {
        println!("No Enemies alive!")
    }
}


fn spawn_wave_box(wave_size: u32, asset_server: &mut Res<AssetServer>, commands: &mut Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/warp.wav"),
        // auto-despawn the entity when playback finishes
        settings: PlaybackSettings::DESPAWN,
    });

    for _ in 1..wave_size { // spawns offset by 1
        
        
        let spawn_x = rand::thread_rng().gen_range( (0. - L_BOUND as f32)..(R_BOUND as f32));
        let spawn_y = rand::thread_rng().gen_range( (T_BOUND as f32)..(T_BOUND as f32 + 200.));

        let rng = rand::thread_rng().gen_range(0..=100);
        match rng {
            0..=20 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Melee, asset_server.load("enemies/melee.png"), health::Health::new(20,150, 3.5,5), MELEE_PATH, LINEAR_DELAY, gun::Gun::new(Vec::new(), 0., 0, 1, 0, 0.)));},
            21..=40 => {
                let mut starting_bullets = Vec::new();
                starting_bullets.push(BULLET_STRAIGHT);
                commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Linear, asset_server.load("enemies/basic.png"), health::Health::new(0,150, 0.0, 5), LINEAR_PATH, LINEAR_DELAY, gun::Gun::new_from_blueprint(starting_bullets, GUN_BLUEPRINT_LINEAR)));
            },
            41..=60 => {
                let mut starting_bullets = Vec::new();
                starting_bullets.push(BULLET_COS_POS);
                starting_bullets.push(BULLET_COS_NEG);
                commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Wavy, asset_server.load("enemies/wavy.png"), health::Health::new(150,150, 3.0, 5), WAVY_PATH, LINEAR_DELAY, gun::Gun::new_from_blueprint(starting_bullets, GUN_BLUEPRINT_WAVY)));
            },
            61..=80 => {
                let mut starting_bullets = Vec::new();
                starting_bullets.push(BULLET_DAIG_NEG_0);
                starting_bullets.push(BULLET_DAIG_NEG_1);
                starting_bullets.push(BULLET_DAIG_POS_0);
                starting_bullets.push(BULLET_DAIG_POS_1);
                commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Spammer, asset_server.load("enemies/spammer.png"), health::Health::new(100,150, 3.0, 5), SPAMMER_PATH, SPAMMER_DELAY, gun::Gun::new_from_blueprint(starting_bullets, GUN_BLUEPRINT_SPAMMER)));
            },
            81..=100 => {
                commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Spawner, asset_server.load("enemies/spawner.png"), health::Health::new(200,250, 3.0, 5), SPAWNER_PATH, SPAWNER_DELAY, gun::Gun::new(Vec::new(), 0., 0, 1, 1000, 2.0)));
            },
            _ => ()

        }

        
        
    }

}



pub fn init_wave(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>
){
        spawn_wave_box(WAVE_SIZE, &mut asset_server, &mut commands);
        commands.insert_resource(WaveTimer(Timer::new(Duration::from_secs_f32(WAVE_INTERVAL), TimerMode::Repeating)));
}

pub fn wave_manager(
    mut commands: Commands,
    mut timer: ResMut<WaveTimer>,
    time: Res<Time>,
    game_time: Res<GameTimer>,
    mut asset_server: Res<AssetServer>
)
{
    timer.0.tick(time.delta());

    if timer.0.finished(){
        
        let minutes_elapsed = (game_time.0.elapsed_secs() / 60.) + 1.;
        let dur = WAVE_INTERVAL as f32 / minutes_elapsed;
        timer.0.set_duration(Duration::from_secs_f32(dur)); // update the wave timer to be smaller

        let size = WAVE_SIZE * (minutes_elapsed + 1.) as u32; // wave size * minutes elapsed
        spawn_wave_box(size, &mut asset_server, &mut commands);

        println!("Spawned wave {}, next interval: {}, Real-Elapsed: {} ", size, dur, game_time.0.elapsed_secs());

        timer.0.reset();
    }

}