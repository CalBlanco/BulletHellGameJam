use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::*};

use crate::{player, enemy};
const BULLET_DEATH: f32 = 5.;

#[derive(Component)]
/// Bullet Struct ;)
pub struct Bullet
{
    dir: i8,
    fx: fn(f32) -> f32,
    fy: fn(f32) -> f32,
    tick: f32,
    ply: bool

} // 

impl Bullet{
    /// Update the life time of the bullet 
    /// 
    pub fn new(dir: i8, fy: fn(f32) -> f32, fx: fn(f32) -> f32, tick: f32, ply: bool) -> Bullet {
        Bullet {dir: dir, fx: fx, fy: fy, tick: tick, ply: ply}
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
    mut sprite_position: Query<(Entity, &mut Bullet, &mut Transform), (With<Bullet>, Without<enemy::Collider>)>,
    collider_query: Query<(Entity, &Transform), (With<enemy::Collider>, Without<Bullet>)>,
    player_query: Query<(Entity, &Transform), (With<player::PlayerControlled>, Without<Bullet>, Without<enemy::Collider>)>,
    mut commands: Commands,
    mut gizmos: Gizmos,
) {
    for (e, mut bullet,  mut b_transform) in &mut sprite_position { // move each bullet 
        // Move the bullet
        if bullet.tick > BULLET_DEATH  {
            commands.entity(e).despawn();
        }

        bullet.update(time.delta_seconds());
        
        b_transform.translation.y += (bullet.fy)(bullet.tick) * bullet.dir as f32; // run the y function
        b_transform.translation.x += (bullet.fx)(bullet.tick) * bullet.dir as f32; // run the x function

        gizmos.rect_2d(b_transform.translation.truncate(), 0., Vec2::new(64., 64.), Color::rgb(1.,0.,0.));

        match bullet.ply {
            false => { // check bullet collision with player
                if let Ok((p_ent, p_transform)) = player_query.get_single() {
                    let collision = bullet_collision(Aabb2d::new(b_transform.translation.truncate(), Vec2::new(32.,32.)), Aabb2d::new(p_transform.translation.truncate(), Vec2::new(64.,64.)));
                
                    if let Some(col) = collision { // collision between enemy and player bullet
                        //if col { commands.entity(p_ent).despawn(); } // despawn the enemy for now 
                        commands.entity(e).despawn();
                    }
                }
            }
            true => {
                // Check bullet collision with enemy
                for(collider_entity, e_transform) in &collider_query {
                    let collision = bullet_collision(Aabb2d::new(b_transform.translation.truncate(), b_transform.scale.truncate()/2.), Aabb2d::new(e_transform.translation.truncate(), Vec2::new(64.,64.)));
                    
                    if let Some(col) = collision { // collision between enemy and player bullet
                        if col { commands.entity(collider_entity).despawn(); commands.entity(e).despawn(); } // despawn the enemy for now 
                        
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