use bevy::prelude::*;
use std::time::Duration;


use crate::{bullet, enemy};



const JUMP_SIZE: f32 = 400.;
const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;
const SPAWN_X: f32 = 0.;
const SPAWN_Y: f32 = -200.;
const MOVE_SPEED: f32 = 180.;
const SHOT_DELAY: f32 = 0.05;

#[derive(Resource)]
pub struct ShotTimer(Timer);


#[derive(Component)]
/// Player information / tag
pub struct PlayerControlled;


// NOT IN USE EVEN THOUGH ITS BEING FUCKING ACCESSED BY THE QUERY TO MOVE IT OH WAIT IT DOESNT EXIST EVEN IF I SPAWN IT IN 80 MILLION TIMES IT JUST REFUSES TO FUCKING ADD THIS TO THE GAME 
// DONT FUCKING NO WHY THIS IS SWAG THO WISH THIS WAS HAPPENING EVERYWHERE 
#[derive(Component)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    control: PlayerControlled
}



impl PlayerBundle {
    pub fn new(spawn_x: f32, spawn_y: f32, asset: Handle<Image>) -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset,
                transform: Transform::from_xyz(spawn_x, spawn_y, 1.),
                ..default()
            },
            control: PlayerControlled
        }
    }

   
}


pub fn sprite_movement(
    time: Res<Time>, 
    mut sprite_position: Query<(Entity, &mut Transform), With<PlayerControlled>>,
    keycode: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gizmos: Gizmos,
    mut shot_timer: ResMut<ShotTimer>
) {
    
    if let Ok((p_ent, mut transform )) = sprite_position.get_single_mut() {
        shot_timer.0.tick(time.delta());

        gizmos.rect_2d(transform.translation.truncate(), 0., Vec2::new(128., 128.), Color::rgb(1.,1.,0.));

        
        if transform.translation.x > R_BOUND as f32 {
            transform.translation.x -= 50.;
        } else if transform.translation.x < -(L_BOUND as f32) {
            transform.translation.x += 50.;
        }
    
        let speed_mult = if keycode.pressed(KeyCode::ShiftLeft){ 3. } else { 1.}; // Speed boost
        let move_dist = MOVE_SPEED * time.delta_seconds() * speed_mult;
        //Move Left
        if keycode.pressed(KeyCode::KeyA) || keycode.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= move_dist; // if transform.translation.x - move_dist < -500. {0.} else {move_dist}
        }
        // Move Right
        if keycode.pressed(KeyCode::KeyD) || keycode.pressed(KeyCode::ArrowRight) {
            transform.translation.x +=  move_dist;//if transform.translation.x + move_dist > R_BOUND as f32 {0.} else {move_dist}
        }
    
        if keycode.just_pressed(KeyCode::KeyS) {
            let jump_x = if transform.translation.x - JUMP_SIZE < 0. - L_BOUND as f32 { 0. - L_BOUND as f32 } else {transform.translation.x - JUMP_SIZE};
            transform.translation.x = jump_x;
        }
        if keycode.just_pressed(KeyCode::KeyW) {
            let jump_x = if transform.translation.x + JUMP_SIZE > L_BOUND as f32 { L_BOUND as f32 } else {transform.translation.x + JUMP_SIZE};
            transform.translation.x = jump_x;
        }
    
       
        // Shoot 
        if keycode.pressed(KeyCode::Space) && shot_timer.0.finished() {
            shot_timer.0.reset();
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| 10.*(a).cos()  ,  0.,  true), asset_server.load("rocket.png")));
            commands.spawn( bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new(1, |_| 50., |_| 0. ,  0., true), asset_server.load("rocket.png")));
        }
    }
    else{
        println!("Player did not spawn");
    }
}


pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    
    let asset = asset_server.load("jet.png");
    
    commands.spawn((
        SpriteBundle {
            texture: asset,
            transform: Transform::from_xyz(SPAWN_X, SPAWN_Y, 1.),
            ..default()
        },
        PlayerControlled)
    );
    commands.insert_resource(ShotTimer(Timer::new(Duration::from_secs_f32(SHOT_DELAY), TimerMode::Repeating)));

}