use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::{bullet, player};

use super::T_BOUND;

const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;


const SHOOT_DELAY: f32 = 0.5;

const MOVE_SPEED: f32 = 125.;

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;



#[derive(Component)]
/// Enemy type enum to determine movement / combat patterns
pub enum EnemyType {
    Melee, // Chase the player attempt to kamakazi them
    Linear, // shoot a straight shot either directly infront or at the player (maybe even fucking random)
    Wavy, // Shoot some cos/sin variant shot 
    Spammer, // shoot massive amounts of shit 
    Spawner, // Shoot some bursts but primarily spawn more of the other types of enemies when killed spawn 2 spawners lol (consequences)
}

#[derive(Component)]
pub struct Enemy {
    t: EnemyType,
    dir: i8,
    last_shot: f32
}


#[derive(Bundle)]
pub struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
    collider: Collider,
    health: player::Health
}

/// Create a new enemey providing a spawn location, type and asset to render
impl EnemyBundle {
    pub fn new(spawn_x: f32, spawn_y: f32, t: EnemyType, asset: Handle<Image>, shield_size: i64, health_size: i64) -> EnemyBundle{
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
                dir: 1,
                t: t,
                last_shot: 0.
            },
            collider: Collider,
            health: player::Health::new(shield_size, health_size)
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
        let random_shot_delay: f32 = rand::thread_rng().gen_range(0.1..=0.525);
        if transform.translation.x <  -(L_BOUND as f32) || transform.translation.x > (R_BOUND as f32) {
            enemy.dir = enemy.dir * -1;
            transform.translation.y -= 96.;
        }

        transform.translation.x += MOVE_SPEED * time.delta_seconds() * enemy.dir as f32;

        
        enemy.last_shot += time.delta_seconds();

        if enemy.last_shot > SHOOT_DELAY && transform.translation.y < T_BOUND as f32{
            enemy.last_shot = 0. - random_shot_delay as f32;
            let spawn_x = transform.translation.x;
            let spawn_y = transform.translation.y - 30.;
            match  enemy.t {
                EnemyType::Melee => {},
                EnemyType::Linear => { // |args| expr == fn(args) {expr}
                    commands.spawn(bullet::BulletBundle::new(spawn_x, spawn_y, bullet::Bullet::new(-1, |_| 20., |_| 0., 0., false, 20), asset_server.load("plasma_red.png")));
                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/laser.wav"),
                        // auto-despawn the entity when playback finishes
                        settings: PlaybackSettings::DESPAWN,
                    });
                },
                EnemyType::Wavy => {
                    commands.spawn(bullet::BulletBundle::new(spawn_x, spawn_y, bullet::Bullet::new(-1, |_| 4.,  |a| 10.*a.cos(), 0.,  false, 60), asset_server.load("plasma_purple.png")));
                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/laser.wav"),
                        // auto-despawn the entity when playback finishes
                        settings: PlaybackSettings::DESPAWN,
                    });
                },
                EnemyType::Spammer => {
                    commands.spawn(bullet::BulletBundle::new(spawn_x, spawn_y, bullet::Bullet::new( -1, |a| 20.*a,  |_| 5., 0., false, 20), asset_server.load("plasma_red.png")));
                    commands.spawn(bullet::BulletBundle::new(spawn_x, spawn_y, bullet::Bullet::new( -1, |a| 20.*a,  |_| -5., 0., false, 20), asset_server.load("plasma_red.png")));
                    commands.spawn(bullet::BulletBundle::new(spawn_x, spawn_y, bullet::Bullet::new( -1, |_| 4.,  |a| 10.*a.cos(), 0., false, 20), asset_server.load("plasma_red.png")));
                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/laser.wav"),
                        // auto-despawn the entity when playback finishes
                        settings: PlaybackSettings::DESPAWN,
                    });
                
                },
                EnemyType::Spawner => {
                    let rng_x = rand::thread_rng().gen_range(1..=5);
                    let rng_y = rand::thread_rng().gen_range(1..=5);
                    spawn_wave_box(rng_x, rng_y, &mut asset_server, &mut commands);
                    enemy.last_shot -= 200.; // Set the spawn timer to have a larger delay than the shoot timer
                    commands.spawn(AudioBundle {
                        source: asset_server.load("sounds/shieldhit.wav"),
                        // auto-despawn the entity when playback finishes
                        settings: PlaybackSettings::DESPAWN,
                    });
                }
    
            }
        }
        
    }
}


fn spawn_wave_box(wave_rows: u32, wave_cols: u32, asset_server: &mut Res<AssetServer>, commands: &mut Commands) {
    for x in 1..wave_rows {
        for y in 0..wave_cols {
            
            let spawn_x =  (64. * x as f32 + 32.) - L_BOUND as f32 as f32;
            let spawn_y = T_BOUND as f32 + (64 * y) as f32 + 64.;

            let rng = rand::thread_rng().gen_range(0..=100);
            match rng {
                0..=25 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Melee, asset_server.load("enemies/melee.png"), 0, 200));},
                26..=50 => {commands.spawn(EnemyBundle  ::new(spawn_x, spawn_y, EnemyType::Linear, asset_server.load("enemies/basic.png"), 100, 50));},
                51..=75 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Wavy, asset_server.load("enemies/wavy.png"), 200, 100));},
                76..=100 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Spammer, asset_server.load("enemies/spammer.png"), 200, 100));},
                _ => ()
            }
        }
    }

    commands.spawn(EnemyBundle::new(0.-L_BOUND as f32, T_BOUND as f32, EnemyType::Spawner, asset_server.load("enemies/spawner.png"), 400, 100)); // always spawn a spawner in the wave
}



pub fn init_wave(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>
){
        spawn_wave_box(10, 4, &mut asset_server, &mut commands)
}
