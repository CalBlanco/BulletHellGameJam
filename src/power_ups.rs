use bevy::{math::bounding::Aabb2d, prelude::*};
use rand::Rng;

use super::B_BOUND;

use crate::{bullet, gun::{self, Gun}, health::Health, player::PlayerControlled, shapes::{self, ShapeBloop, ShapeGun}};

#[derive(Component, Copy, Clone)]
pub enum PowerUpTypes { 
    BulletAmmo,
    BulletSpeed,
    BulletDamage,
    AddRandomBullet,
    ShapeAmmo,
    ShapeReloadTime,
    AddRandomShape,
    ShapeSize,
    HealthIncrease,
    ShieldIncrease,
    ShieldRegen,
}

impl PowerUpTypes {
    fn value(&self) -> &str {
        match self {
            PowerUpTypes::BulletAmmo => "power_ups/bullets/bullet_ammo.png",
            PowerUpTypes::BulletSpeed => "power_ups/bullets/bullet_speed.png",
            PowerUpTypes::BulletDamage => "power_ups/bullets/bullet_attack.png",
            PowerUpTypes::AddRandomBullet => "power_ups/bullets/bullet_random.png",
            PowerUpTypes::ShapeAmmo => "power_ups/shapes/shape_ammo.png",
            PowerUpTypes::ShapeReloadTime => "power_ups/shapes/shape_reload_speed.png",
            PowerUpTypes::AddRandomShape => "power_ups/shapes/shape_random.png",
            PowerUpTypes::ShapeSize => "power_ups/shapes/shape_inc_size.png",
            PowerUpTypes::HealthIncrease => "power_ups/health/health_increase.png",
            PowerUpTypes::ShieldIncrease => "power_ups/health/shield_increase.png",
            PowerUpTypes::ShieldRegen => "power_ups/health/shield_speed.png",
            
        }
    }
}


#[derive(Bundle)]
pub struct PowerUpBundle {
    sprite: SpriteBundle,
    p_type: PowerUpTypes
}


/// Spawn 3 random power ups at the top of the screen 
pub fn spawn_powerup_wave(coms: &mut Commands, assets: &Res<AssetServer>){
    // pick a random bullet powerup
    // a random shape power up,
    // and a random health power up 

    let bullet_pups = vec![PowerUpTypes::BulletAmmo, PowerUpTypes::BulletDamage, PowerUpTypes::BulletSpeed, PowerUpTypes::BulletAmmo, PowerUpTypes::BulletDamage, PowerUpTypes::BulletSpeed,  PowerUpTypes::AddRandomBullet];
    let shape_pups = vec![PowerUpTypes::ShapeAmmo, PowerUpTypes::ShapeReloadTime, PowerUpTypes::ShapeSize, PowerUpTypes::ShapeAmmo, PowerUpTypes::ShapeReloadTime, PowerUpTypes::ShapeSize, PowerUpTypes::AddRandomShape];
    let health_pups = vec![PowerUpTypes::HealthIncrease, PowerUpTypes::ShieldIncrease, PowerUpTypes::ShieldRegen];

    let bup = rand::thread_rng().gen_range(0..(bullet_pups.len()));
    let sup = rand::thread_rng().gen_range(0..(shape_pups.len()));
    let hup = rand::thread_rng().gen_range(0..(health_pups.len()));

    let bup = bullet_pups[bup];
    let sup = shape_pups[sup];
    let hup = health_pups[hup];

    let points = [(-460., 400.), (0., 400.) ,(460., 400.)];
    let pups = [bup, sup, hup];

    for n in 0..pups.len() {
        let asset = pups[n];
        let asset = asset.value().to_owned();
        coms.spawn(PowerUpBundle {
            sprite: SpriteBundle {
                texture: assets.load(asset),
                transform: Transform::from_xyz(points[n].0, points[n].1, 1.0),
                ..default()
            },
            p_type: pups[n]
        });
    }
}

const MOVE_SPEED: f32 = 180.;

pub fn move_powerups(
    mut power_ups: Query<(Entity, &mut Transform), With<PowerUpTypes>>,
    mut coms: Commands,
    time: Res<Time>
){
    for (ent, mut transform) in &mut power_ups{
        if transform.translation.y < B_BOUND {
            coms.entity(ent).despawn();
        }

        transform.translation.y -= MOVE_SPEED * time.delta_seconds();
    }
}


/// Move and collide power ups 
pub fn handle_powerup_collision(
    mut player: Query<(&mut Health, &mut Gun, &mut ShapeGun, &Transform), With<PlayerControlled>>,
    power_ups: Query<(Entity, &PowerUpTypes, &Transform), With<PowerUpTypes>>,
    mut coms: Commands
){
    if let Ok((mut health, mut gun, mut shape_gun, p_transform)) = player.get_single_mut() {
        let mut did_contact = false;
        for (ent, power_up, transform) in &power_ups {
            if did_contact {break;}
            let collision = bullet::bullet_collision(Aabb2d::new(p_transform.translation.truncate(), Vec2::new(16.,16.)), Aabb2d::new(transform.translation.truncate(), Vec2::new(16.,16.)));
            if let Some(_) = collision {
                match power_up {
                    PowerUpTypes::BulletAmmo => {
                        let cur = gun.get_max_ammo();
                        gun.set_max_ammo(cur + 100);
                    },
                    PowerUpTypes::BulletSpeed => {
                        let cur = gun.get_bullet_delay();
                        gun.set_bullet_delay(cur - 0.005)
                    },
                    PowerUpTypes::BulletDamage => {
                        let cur = gun.get_bullet_damage();
                        gun.set_bullet_damage(cur + 50);
                    },
                    PowerUpTypes::AddRandomBullet => {
                        let b_choice = rand::thread_rng().gen_range(0..5);
                        match b_choice {
                            0 => gun.add_bullet(gun::BulletBlueprint(1, |y| y*y, |_| 0., 0., true, 50)),
                            1 => gun.add_bullet(gun::BulletBlueprint(1, |y| y*y, |_| 5., 0., true, 50)),
                            2 => gun.add_bullet(gun::BulletBlueprint(1, |y| y*y, |_| -5., 0., true, 50)),
                            3 => gun.add_bullet(gun::BulletBlueprint(1, |_| 10., |_| 5., 0., true, 50)),
                            _ => gun.add_bullet(gun::BulletBlueprint(1, |_| 10., |_| -5., 0., true, 50))
                        }
                    },
                    PowerUpTypes::ShapeAmmo => {
                        let shots = shape_gun.get_max_shots() + 1;
                        shape_gun.set_max_shots(shots);
                    },
                    PowerUpTypes::ShapeReloadTime => {
                        let cur = shape_gun.timer.duration().as_secs_f32();
                        shape_gun.set_reload_time(cur - 0.05);
                    },
                    PowerUpTypes::AddRandomShape => {
                        let s_choice = rand::thread_rng().gen_range(0..3);
                        let x_off = rand::thread_rng().gen_range(-200. .. 200.);
                        let y_off = rand::thread_rng().gen_range(0. .. 150.);

                        let offset = (x_off, y_off);
                        
                        let x_scale = rand::thread_rng().gen_range(0.5 .. 3.0);
                        let y_scale = rand::thread_rng().gen_range(0.5 .. 3.0);
                        let scale = (x_scale, y_scale);
                        
                        match s_choice{
                            0 => {
                                shape_gun.add_bloop(ShapeBloop{ offset: offset, num_bullets: 50, t: shapes::ShapeType::Circle, size_scale: scale});
                            },
                            1 => {
                                shape_gun.add_bloop(ShapeBloop{ offset: offset, num_bullets: 50, t: shapes::ShapeType::Triangle, size_scale: scale});
                            },
                            _ => {
                                shape_gun.add_bloop(ShapeBloop{ offset: offset, num_bullets: 50, t: shapes::ShapeType::Square, size_scale: scale});
                            }
                        }
                    },
                    PowerUpTypes::ShapeSize => {
                        let cur = shape_gun.get_size();
                        shape_gun.set_size(cur * 1.05);
                    },
                    PowerUpTypes::HealthIncrease => {
                        let cur = health.get_health();
                        health.set_health(cur + 100);
                    },
                    PowerUpTypes::ShieldIncrease => {
                        let cur = health.get_max_shield();
                        health.set_max_shield(cur + 100);
                    },
                    PowerUpTypes::ShieldRegen => {
                        let cur = health.get_recharge();
                        health.set_recharge(cur + 50);
                    },
                    
                }
            
                coms.entity(ent).despawn();
                did_contact = true;
            }
        }

        if did_contact {
            for (e, _, _) in &power_ups {
                coms.entity(e).despawn();
            }
        }
    }

    

}

