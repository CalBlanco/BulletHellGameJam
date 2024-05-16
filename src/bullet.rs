use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};
use bevy_hanabi::{EffectProperties, EffectSpawner};
use rand::Rng;

use crate::{enemy, game::ScoreBoard, gun::{self, BulletBlueprint}, health, player::{self, PlayerControlled}};
use super::{T_BOUND, B_BOUND, L_BOUND, R_BOUND};

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

        if b_transform.translation.x < 0. - L_BOUND as f32 && bullet.ply {
            b_transform.translation.x = R_BOUND as f32 - 1.;
        }
        if b_transform.translation.x > R_BOUND as f32 && bullet.ply {
            b_transform.translation.x = 0. - L_BOUND as f32 + 1.; 
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


fn explosion_collision(circle: BoundingCircle, square: Aabb2d) -> Option<bool> {
    if circle.intersects(&square) {
        return Some(true)
    }else {
        return None
    }
}


//Its really not good im doing all this inside this function lmao
/// Event for processing damage
pub fn apply_collision_damage(
    mut health_query: Query<&mut health::Health>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut score_events: EventWriter<ScoreEvent>,
    enemy_query: Query<(Entity, &enemy::Enemy, &Transform), (With<enemy::Collider>, Without<EffectProperties>)>,
    mut gun_query: Query<&mut gun::Gun, With<player::PlayerControlled>>,
    mut effect: Query<(
        &mut EffectProperties,
        &mut EffectSpawner,
        &mut Transform,
    )>,
    asset_server: Res<AssetServer>
){
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        for dmg in collision_events.read() {
            if let Ok(mut health) = health_query.get_mut(dmg.0) {
                health.damage(dmg.1);
                

                if !health.is_alive() { // Entity has died from damage
                    //check if we should add score
                    if !dmg.2 { // not a player dying 
                        

                        if let Ok((_, en, transform)) = enemy_query.get(dmg.0) { // enemy killed 

                            commands.spawn(AudioBundle {
                                source: asset_server.load("sounds/hit.wav"),
                                // auto-despawn the entity when playback finishes
                                settings: PlaybackSettings::DESPAWN
                            });

                            let (score, mul) = en.get_type().get_score();
                            score_events.send(ScoreEvent(score, mul));

                            if let Ok(mut gun) = gun_query.get_single_mut() { // Gun Upgrades from kills
                                let gun_damage = gun.get_bullet_damage(); // get gun damage and speed 
                                let gun_speed: f32 = gun.get_bullet_delay();

                                match en.get_type() { // match type for reward / consequence 
                                    enemy::EnemyType::Spawner => {
                                       
                                        let b_choice = rand::thread_rng().gen_range(0..5);
                                        match b_choice {
                                            0 => gun.add_bullet(gun::BulletBlueprint(1, |y| y*y, |_| 0., 0., true, 50)),
                                            1 => gun.add_bullet(gun::BulletBlueprint(1, |y| y*y, |_| 5., 0., true, 50)),
                                            2 => gun.add_bullet(gun::BulletBlueprint(1, |y| y*y, |_| -5., 0., true, 50)),
                                            3 => gun.add_bullet(gun::BulletBlueprint(1, |_| 10., |_| 5., 0., true, 50)),
                                            _ => gun.add_bullet(gun::BulletBlueprint(1, |_| 10., |_| -5., 0., true, 50))
                                        }
                                        
                                    }
                                    enemy::EnemyType::Wavy => {
                                        gun.set_bullet_damage(gun_damage + 30)
                                    },
                                    enemy::EnemyType::Spammer => {
                                        gun.set_bullet_delay(gun_speed - 0.0005)
                                    },
                                    enemy::EnemyType::Linear => {},
                                    enemy::EnemyType::Melee => {}
                                }    
                            }

                            let Ok((
                                mut properties,
                                mut spawner,
                                mut effect_transform,
                            )) = effect.get_single_mut()
                            else {
                                warn!("effect not ready yet, returning");
                                return;
                            };

                            let color = Color::lch(1., 1., rand::random::<f32>() * 120.);
                            properties.set(
                                "spawn_color",
                                color.as_linear_rgba_u32().into(),
                            );

                            effect_transform.translation = transform.translation;
                            spawner.reset(); // spawn the effect 


                            for (e, _, effected_transform) in enemy_query.iter() {
                                if let Some(_) = explosion_collision(BoundingCircle::new(transform.translation.truncate(), 32.), Aabb2d::new(effected_transform.translation.truncate(), effect_transform.scale.truncate()/2.)){
                                    let Ok(mut e_health) = health_query.get_mut(e) else {return;};
                                    e_health.damage(20);
                                }
                            }
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
    mut player_gun: Query<&mut gun::Gun, With<PlayerControlled>>
){
    if !score_events.is_empty() {
        for score in score_events.read() {
            
            println!("Adding Score: {} - {} ", score.0 ,scoreboard.get_score());

            scoreboard.add_score(score.0);
            scoreboard.add_mul(score.1);

            let Ok(mut pg) = player_gun.get_single_mut() else {return;};
            let mul = (scoreboard.get_mul() + 3) as i64;
            pg.set_bullet_damage(35 * std::cmp::min(mul, 16));
        }
    }
}



pub fn bullet_on_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &mut Bullet, &mut Transform)>,
)
{   
    // 
    let mut ply_bullets = Vec::new();
    let mut en_bullets = Vec::new();

    for (en, bul, transform) in bullet_query.iter() {
        if bul.ply {ply_bullets.push((en,bul,transform))} else {en_bullets.push((en,bul,transform));}
    }

    for (en, _, tran) in ply_bullets {
        for(en2, _, tran2) in &en_bullets{
            
 
            let collision = bullet_collision(Aabb2d::new(tran.translation.truncate(), Vec2::new(8.,8.)), Aabb2d::new(tran2.translation.truncate(), Vec2::new(8.,8.)));

            if let Some(_) = collision { // collision between enemy and player bullet
                commands.entity(*en2).despawn(); // despawn the bullet 
                commands.entity(en).despawn(); // despawn the bullet 
                break; // exit this loop so we iterate out of the outer after removing its bullet
            }
            continue;

        }
    }
}




