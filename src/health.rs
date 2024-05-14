use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct ShieldTimer(Timer);


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

    pub fn shield_tick(&mut self, dur: Duration) {
        self.timer.0.tick(dur);
    }

    pub fn can_shield_recharge(&self) -> bool {
        self.timer.0.finished()
    }

  
}



pub fn shield_tick(
    time: Res<Time>,
    mut query: Query<&mut Health>
){
    for mut health in query.iter_mut(){
        health.shield_tick(time.delta()); // increment timer

        if health.can_shield_recharge() {
            health.regen_shield(3);
        }

    }
}