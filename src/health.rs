use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct ShieldTimer(pub Timer);


#[derive(Component)]
pub struct Health {
    shield: i64,
    health: i64,
    is_alive: bool,
    pub timer: ShieldTimer,
    shield_recharge: i64,
    max_shield: i64,
    max_health: i64
    //max_health: i64,
}

impl Health {
    /// Create a new health component specifying shield size, and health
    pub fn new(shield_size: i64, health_size: i64, shield_time: f32, shield_recharge: i64) -> Health {
        Health {
            shield: shield_size,
            max_health: health_size,
            health: health_size,
            is_alive: true,
            timer: ShieldTimer(Timer::new(Duration::from_secs_f32(shield_time), TimerMode::Once)),
            shield_recharge: shield_recharge,
            //max_health: health_size,
            max_shield: shield_size
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
    pub fn get_max_health(&self) -> i64 {self.max_health}
    pub fn get_shield(&self) -> i64 {self.shield}
    pub fn get_max_shield(&self) -> i64 { self.max_shield}

    pub fn recharge_shield(&mut self){
        self.shield =  if self.shield + self.shield_recharge <= self.max_shield {self.shield + self.shield_recharge}  else {self.max_shield};
    }

    pub fn shield_tick(&mut self, dur: Duration) {
        self.timer.0.tick(dur);
    }

    pub fn can_shield_recharge(&self) -> bool {
        self.timer.0.finished()
    }

    pub fn set_max_health(&mut self, health: i64){
        self.max_health = health;
    }

    pub fn set_max_shield(&mut self, shield: i64){
        self.max_shield = shield;
    }

    pub fn set_recharge(&mut self, recharge: i64){
        self.shield_recharge = recharge;
    }

    pub fn get_recharge(&self) -> i64 { self.shield_recharge }
 

  
}


/// Check if the shield can recharge and then recharge it 
pub fn shield_tick(
    time: Res<Time>,
    mut query: Query<&mut Health>
){
    for mut health in query.iter_mut(){
        health.shield_tick(time.delta()); // increment timer

        if health.can_shield_recharge() {
            health.recharge_shield();
        }

    }
}


