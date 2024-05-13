use bevy::{audio::Volume, math::bounding::{Aabb2d, IntersectsVolume}, prelude::*};

use crate::{enemy, game::ScoreBoard, player, PLAYBACK_SPEED};
use super::{T_BOUND, B_BOUND};

const BULLET_DEATH: f32 = 5.;

#[derive(Event)]
pub struct CollisionEvent(Entity, i64, bool);

#[derive(Event)]
pub struct ScoreEvent(u64, u64);  // add to score on a event (add this event to event queue when a unit dies if it is not the player???)


#[derive(Component)]
/// Bullet Struct ;)
pub struct Bullet
{
    dir: i8,
    fx: fn(f32) -> f32,
    fy: fn(f32) -> f32,
    tick: f32,
    ply: bool,
    damage: i64
} // 

impl Bullet{
    /// Update the life time of the bullet 
    /// 
    pub fn new(dir: i8, fy: fn(f32) -> f32, fx: fn(f32) -> f32, tick: f32, ply: bool, damage: i64) -> Bullet {
        Bullet {dir: dir, fx: fx, fy: fy, tick: tick, ply: ply, damage: damage}
    }
    pub fn update(&mut self, time:f32) {
        self.tick += time;
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    sprite_bundle: SpriteBundle,
    bullet: Bullet,
}

/// Bundle to contain the bullet class
impl BulletBundle{
    /// Create a new bullet by passing a spawn position, bullet data (direction, fx, fy, and tick start), and a texture image from the Asset loader
    pub fn new(spawn_x: f32, spawn_y: f32, bullet: Bullet, asset: Handle<Image>) -> BulletBundle {
        BulletBundle {
            sprite_bundle: SpriteBundle {
                texture: asset,
                transform: Transform::from_xyz(spawn_x, spawn_y, 0.),
                ..default()
            },
            bullet: bullet
        }
    }
}


/// Move the bullets 
pub fn bullet_movement(
    time: Res<Time>, 
    mut bullet_query: Query<(Entity, &mut Bullet, &mut Transform), (With<Bullet>, Without<enemy::Collider>)>,
    collider_query: Query<(Entity, &Transform), (With<enemy::Collider>, Without<Bullet>)>,
    player_query: Query<(Entity, &Transform), (With<player::PlayerControlled>, Without<Bullet>, Without<enemy::Collider>)>,
    mut commands: Commands,
    mut collision_events: EventWriter<CollisionEvent>,
    mut scoreboard: ResMut<ScoreBoard>
) {
    for (bullet_entity, mut bullet,  mut b_transform) in &mut bullet_query { // move each bullet 
        // Move the bullet
        if bullet.tick > BULLET_DEATH || b_transform.translation.y < B_BOUND || b_transform.translation.y > (T_BOUND + 64) as f32 {
            commands.entity(bullet_entity).despawn();
            continue;
        }

        bullet.update(time.delta_seconds());
        
        b_transform.translation.y += (bullet.fy)(bullet.tick) * bullet.dir as f32; // run the y function
        b_transform.translation.x += (bullet.fx)(bullet.tick) * bullet.dir as f32; // run the x function

        //gizmos.rect_2d(b_transform.translation.truncate(), 0., Vec2::new(16., 16.), Color::rgb(1.,0.,0.));

        match bullet.ply {
            false => { // check bullet collision with player
                if let Ok((p_ent, p_transform)) = player_query.get_single() {
                    let collision = bullet_collision(Aabb2d::new(b_transform.translation.truncate(), Vec2::new(8.,8.)), Aabb2d::new(p_transform.translation.truncate(), Vec2::new(16.,16.)));
                    
                    if let Some(_) = collision { // collision between enemy and player bullet
                        // want to fire sound here  
                        collision_events.send(CollisionEvent(p_ent, bullet.damage, true));
                        scoreboard.set_mul(0); // reset player multiplier when they are hit
                        commands.entity(bullet_entity).despawn(); // despawn the bullet 
                    }
                }
            }
            true => {
                // Check bullet collision with enemy
                for(collider_entity, e_transform) in &collider_query {
                    let collision = bullet_collision(Aabb2d::new(b_transform.translation.truncate(), b_transform.scale.truncate()/2.), Aabb2d::new(e_transform.translation.truncate(), Vec2::new(16.,16.)));
                    
                    if let Some(_) = collision { // collision between enemy and player bullet
                        collision_events.send(CollisionEvent(collider_entity, bullet.damage, false));
                        commands.entity(bullet_entity).despawn(); // despawn the bullet 
                    }
                }
            }
        }
        
    }
}

/// Check if a bullet has intersected a enemy / bounding box 
fn bullet_collision(bullet: Aabb2d, enemy: Aabb2d) -> Option<bool> {
    if bullet.intersects(&enemy) {
        return Some(true)
    }else {
        return None
    }

    
}


pub fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/hit.wav"),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(0.25),
                speed: PLAYBACK_SPEED,
                ..default()
            },
        });
    }
}

/// Event for processing damage
pub fn apply_collision_damage(
    mut health_query: Query<&mut player::Health>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut score_events: EventWriter<ScoreEvent>,
    enemy_query: Query<&enemy::Enemy, With<enemy::Collider>>,
){
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        for dmg in collision_events.read() {
            if let Ok(mut health) = health_query.get_mut(dmg.0) {
                health.damage(dmg.1);

                if !health.is_alive() { // Entity has died from damage
                    //check if we should add score
                    println!("Should add Score? {}", !dmg.2);
                    println!("Found {} enemies in query ", enemy_query.iter().len());
                    if !dmg.2 { // not a player dying 
                        if let Ok(en) = enemy_query.get(dmg.0) {
                            let (score, mul) = en.get_type().get_score();
                            score_events.send(ScoreEvent(score, mul));    
                        }
                        
                    }    

                    commands.entity(dmg.0).despawn(); // despawn
                }
            }
            
        }

        collision_events.clear(); // empty out when done
    }
}


pub fn update_score(
    mut score_events: EventReader<ScoreEvent>,
    mut scoreboard: ResMut<ScoreBoard>,
){
    if !score_events.is_empty() {
        for score in score_events.read() {
            
            println!("Adding Score: {} - {} ", score.0 ,scoreboard.get_score());

            scoreboard.add_score(score.0);
            scoreboard.add_mul(score.1);
        }
    }
}
