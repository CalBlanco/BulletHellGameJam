use bevy::{audio::Volume, prelude::*};



use crate::{bullet, game, gun, health, shapes, GameState, PLAYBACK_SPEED, PLAYBACK_VOL};

use super::{EzTextBundle, B_BOUND, L_BOUND, R_BOUND};




const PLAYER_T_BOUND: f32 = -200.;
const SPAWN_X: f32 = 0.;
const SPAWN_Y: f32 = B_BOUND + 100.;

const MOVE_SPEED: f32 = 180.;
const SHOT_DELAY: f32 = 0.08;


const SHIELD_SIZE: i64 = 32_500;
const HEALTH_SIZE: i64 = 32_500;

const BULLET_DAMAGE: i64 = 20;


// TODO: implement a shield reset timer

#[derive(Component)]
/// Player information / tag
pub struct PlayerControlled;


#[derive(Component)]
pub struct HealthText;
#[derive(Component)]
pub struct ShieldText;
#[derive(Component)]
pub struct ScoreText;


pub fn sprite_movement(
    time: Res<Time>, 
    mut sprite_position: Query<(Entity, &mut Transform, &mut gun::Gun), With<PlayerControlled>>,
    keycode: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    
    if let Ok((_, mut transform , mut gun)) = sprite_position.get_single_mut() {
        gun.tick_time(time.delta());

        //gizmos.rect_2d(transform.translation.truncate(), 0., Vec2::new(32., 32.), Color::rgb(1.,1.,0.));

        // Bound X
        if transform.translation.x > R_BOUND as f32 {
            transform.translation.x  = 1. - L_BOUND as f32 ;
        } else if transform.translation.x < -(L_BOUND as f32) {
            transform.translation.x = (R_BOUND - 1) as f32;
        }
        // Bound Y 
        if transform.translation.y > PLAYER_T_BOUND as f32 {
            transform.translation.y  = (PLAYER_T_BOUND - 1.) as f32;
        } else if transform.translation.y < SPAWN_Y {
            transform.translation.y = SPAWN_Y + 1.; 
        }


    
        let speed_mult = if keycode.pressed(KeyCode::ShiftLeft){ 3. } else { 1.}; // Speed boost
        let move_dist = MOVE_SPEED * time.delta_seconds() * speed_mult;
        //Move Left
        if keycode.pressed(KeyCode::KeyA) {
            transform.translation.x -= move_dist; // if transform.translation.x - move_dist < -500. {0.} else {move_dist}
        }
        // Move Right
        if keycode.pressed(KeyCode::KeyD)  {
            transform.translation.x +=  move_dist;//if transform.translation.x + move_dist > R_BOUND as f32 {0.} else {move_dist}
        }
    
        if keycode.pressed(KeyCode::KeyW){ 
            transform.translation.y += move_dist;
        }
        if keycode.pressed(KeyCode::KeyS){
            transform.translation.y -= move_dist;
        }
    
        if keycode.just_pressed(KeyCode::KeyE){
            
            let points = shapes::generate_line(transform.translation.x - 100., transform.translation.y - 100., transform.translation.x + 100., transform.translation.y + 100., 30);
            for p in points {  
                commands.spawn(bullet::BulletBundle::new(p.0, p.1, bullet::Bullet::new(1, |y| y*y, |x| 3.*(7.*x).cos(), 0., true, 20), asset_server.load("plasma_green.png")));
            }
        }

       
        // Shoot 
        if keycode.pressed(KeyCode::Space) && gun.can_shoot() {
            gun.reset_shot_timer();
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/laser.wav"),
                // auto-despawn the entity when playback finishes
                settings: PlaybackSettings::DESPAWN
            });

            

            let bullets = gun.get_bullets();

            for bul in bullets {
                commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new(bul.0, bul.1, bul.2, bul.3, bul.4, gun.get_bullet_damage()), asset_server.load("plasma_blue.png")));
            }

            /* commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| 5.*(a).cos()  ,  0.,  true, bullet_damage), asset_server.load("plasma_blue.png")));
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 3., |a: f32| -5.*(a).cos()  ,  0.,  true, bullet_damage), asset_server.load("plasma_blue.png")));
            commands.spawn(bullet::BulletBundle::new(transform.translation.x, transform.translation.y, bullet::Bullet::new( 1, |_| 30., |_| 0.  ,  0.,  true, bullet_damage), asset_server.load("plasma_green.png"))); */
        }
    }
    else{
        game_state.set(GameState::Menu); // restart game when player is unfindable (dead)
    }
}


#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    control: PlayerControlled,
    health: health::Health,
    gun: gun::Gun
}

impl PlayerBundle {
    fn new(asset: Handle<Image>) -> PlayerBundle {
        let mut starting_bullets = Vec::new();
        starting_bullets.push(gun::BulletBlueprint(1,|_| 5., |x| 9.*(x*10.).cos(), 0., true, 50));
        starting_bullets.push(gun::BulletBlueprint(1,|_| 5., |x| -9.*(x*10.).cos(), 0., true, 50));
        starting_bullets.push(gun::BulletBlueprint(1,|_| 20., |_: f32| 0., 0., true, 50));
       
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset,
                transform: Transform::from_xyz(SPAWN_X, SPAWN_Y, 1.),
                ..default()
            },
            control: PlayerControlled,
            health: health::Health::new(SHIELD_SIZE, HEALTH_SIZE, 3.75, 100),
            gun: gun::Gun::new(starting_bullets, SHOT_DELAY, BULLET_DAMAGE, 20),
        }
    } 
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    
    let asset = asset_server.load("player.png");
    
    commands.spawn(
        PlayerBundle::new(asset)
    );


    commands.spawn(EzTextBundle::new(String::from("Health: "), 60., 820., 20., asset_server.load("fonts/Lakmus.ttf"), Color::TEAL,HealthText));
    commands.spawn(EzTextBundle::new(String::from("Shield: "), 60., 880., 20., asset_server.load("fonts/Lakmus.ttf"), Color::TEAL,ShieldText));
    commands.spawn(EzTextBundle::new(String::from("Score: "), 80., 760., 20., asset_server.load("fonts/Lakmus.ttf"), Color::GOLD,ScoreText));

}

pub fn update_player_health(mut query: Query<&mut Text, With<HealthText>>, mut player: Query<&mut health::Health, With<PlayerControlled>>){
    for mut text in &mut query {
        if let Ok(health) = player.get_single_mut() {
            text.sections[0].value = format!("Health: {:.2}", health.get_health());
        }
    }
}

pub fn update_player_shield(mut query: Query<&mut Text, With<ShieldText>>, mut player: Query<&mut health::Health, With<PlayerControlled>>){
    for mut text in &mut query {
        if let Ok(health) = player.get_single_mut() {
            text.sections[0].value = format!("Shield: {:.2}", health.get_shield());
        }
    }
}

pub fn update_player_score(
    mut query: Query<&mut Text, With<ScoreText>>, 
    scoreboard: Res<game::ScoreBoard>
) {
    for mut text in &mut query {
        text.sections[0].value = format!("{:09} x {:01}", scoreboard.get_score(), scoreboard.get_mul())
    }
}

