use std::time::Duration;

use bevy::prelude::*;


#[derive(Resource)]
/// Timer for shots (does not apply to AI as they have a range used by their own logic)
pub struct ShotTimer(Timer);


// Implement a bullet blueprint allowing for quick instantiation of bullet objects without taking space on the heap until needed
/// Bullet blueprint for constant storage of certain types:  (**dir**:*i8*, **fy**:*fn(f32)->f32*, **fx**:*fn(f32)->f32*, **tick**:*f32*, **team**:*bool*, **damage**:*i64*)
pub struct BulletBlueprint(pub i8, pub fn(f32)->f32, pub fn(f32)->f32, pub f32, pub bool, pub i64);

/// Gun Blueprint for constant storage of certain gun types:  (**shoot_delay**:*f32*, **damage**:*i64*, **max_bullets**:*u8*, **max_ammo**:*usize*, **reload_delay**:*usize*)
pub struct GunBluePrint(pub f32, pub i64, pub u8, pub u64, pub f32);

#[derive(Component)]
/// 
pub struct Gun {
    bullet_blueprints: Vec<BulletBlueprint>,
    damage: i64,
    shoot_delay: f32,
    max_bullets: u8,
    pub shot_timer: ShotTimer,
    ammo: u64,
    max_ammo: u64,
    pub reload_stopwatch: Timer,
    reload_delay: f32
}

impl Gun {
    pub fn new(starting_bullets: Vec<BulletBlueprint>, init_shoot_delay: f32, init_damage: i64, max_bullets: u8, ammo: u64, reload_delay: f32) -> Gun{    
        Gun {
            bullet_blueprints: if starting_bullets.is_empty() { Vec::new()} else {starting_bullets},
            damage: init_damage,
            shoot_delay: init_shoot_delay,
            max_bullets: max_bullets,
            shot_timer: ShotTimer(Timer::from_seconds(init_shoot_delay, TimerMode::Repeating)),
            ammo: ammo,
            max_ammo: ammo,
            reload_stopwatch: Timer::new(Duration::from_secs_f32(reload_delay), TimerMode::Once),
            reload_delay: reload_delay
        }
    }

    pub fn new_from_blueprint(starting_bullets: Vec<BulletBlueprint>, gun_blueprint: GunBluePrint) -> Gun {
        Gun {
            bullet_blueprints: if starting_bullets.is_empty() { Vec::new()} else {starting_bullets},
            damage: gun_blueprint.1,
            shoot_delay: gun_blueprint.0,
            max_bullets: gun_blueprint.2,
            shot_timer: ShotTimer(Timer::from_seconds(gun_blueprint.0, TimerMode::Repeating)),
            ammo: gun_blueprint.3,
            max_ammo: gun_blueprint.3,
            reload_stopwatch: Timer::new(Duration::from_secs_f32(gun_blueprint.4), TimerMode::Once),
            reload_delay: gun_blueprint.4
        }
    }

    pub fn add_bullet(&mut self, blueprint: BulletBlueprint){
        if self.bullet_blueprints.len() < self.max_bullets as usize {
            self.bullet_blueprints.push(blueprint)
        }
    }

    pub fn get_bullets(&self) -> &Vec<BulletBlueprint> { &self.bullet_blueprints }

    pub fn set_bullet_delay(&mut self, new_delay: f32) {

        self.shoot_delay = if  new_delay >= 0.03 {new_delay} else {0.03};
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
        self.shot_timer.0.finished() && self.ammo > 0
    }

    pub fn reset_shot_timer(&mut self){
        self.shot_timer.0.reset();
    }

    pub fn tick_reload_time(&mut self, time: Duration){
        self.reload_stopwatch.tick(time);
    }

    pub fn can_reload(&self) -> bool{
        self.reload_stopwatch.finished()
    }

    pub fn set_reload_delay(&mut self, time: f32){
        self.reload_stopwatch.set_duration(Duration::from_secs_f32(time))
    }

    pub fn reload(&mut  self){
        self.ammo = self.max_ammo;
        self.reload_stopwatch.reset();
    }

    pub fn set_max_ammo(&mut self, max_ammo: u64){
        self.max_ammo = max_ammo
    }

    pub fn shoot_bullet(&mut self){
        self.ammo = if self.ammo - 1 > 0 {self.ammo - 1} else { 0 };
    }

    pub fn get_ammo(&self) -> u64{
        self.ammo
    }
    pub fn get_max_ammo(&self) -> u64{
        self.max_ammo
    }

    pub fn set_ammo(&mut self, ammo: u64){
        self.ammo = ammo;
    }
}
