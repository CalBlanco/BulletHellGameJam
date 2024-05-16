use std::time::Duration;

use bevy::prelude::*;



use crate::{bullet, game, gun, health, shapes::{self, ShapeBloop}, GameState};

use super::{EzTextBundle, B_BOUND, L_BOUND, R_BOUND};




const PLAYER_T_BOUND: f32 = -200.;
const SPAWN_X: f32 = 0.;
const SPAWN_Y: f32 = B_BOUND + 100.;

const MOVE_SPEED: f32 = 180.;
const SHOT_DELAY: f32 = 0.08;


const SHIELD_SIZE: i64 = 500;
const HEALTH_SIZE: i64 = 500;

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

#[derive(Component)]
pub struct TimeText;

#[derive(Component)]
pub struct AmmoText;

pub fn sprite_movement(
    time: Res<Time>, 
    mut sprite_position: Query<(Entity, &mut Transform, &mut gun::Gun, &mut shapes::ShapeGun), With<PlayerControlled>>,
    keycode: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    
    if let Ok((_, mut transform , mut gun, mut s_gun)) = sprite_position.get_single_mut() {
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

        if keycode.just_pressed(KeyCode::KeyR) {
            gun.set_ammo(0);
            gun.reload_stopwatch.reset();
            gun.reload_stopwatch.tick(Duration::from_secs_f32(1.5));
        }
    
        if keycode.just_pressed(KeyCode::KeyE) && s_gun.get_shots() > 0 {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/womp.wav"),
                // auto-despawn the entity when playback finishes
                settings: PlaybackSettings::DESPAWN
            });
            
            s_gun.shoot();
            let px = transform.translation.x;
            let py = transform.translation.y;
            let points = s_gun.get_shapes(px, py);

            let bloop = &s_gun.bullet;
    
            for p in points {  
                commands.spawn(bullet::BulletBundle::new(p.0, p.1, bullet::Bullet::new(bloop.0, bloop.1, bloop.2, bloop.3, bloop.4, bloop.5), asset_server.load("plasma_green.png")));
            }
                
            
            
        }

       
        // Shoot 
        if keycode.pressed(KeyCode::Space) && gun.can_shoot() {
            gun.reset_shot_timer();
            gun.shoot_bullet();

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
    
        if gun.get_ammo() <= 0 {
            gun.tick_reload_time(time.delta());

            if gun.can_reload() {
                gun.reload();
            }
        }

        if s_gun.get_shots() <= 0 {
            s_gun.timer.tick(time.delta());

            if s_gun.timer.finished() {
                s_gun.reload();
            }
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
    gun: gun::Gun,
    s_gun: shapes::ShapeGun
    
}

impl PlayerBundle {
    fn new(asset: Handle<Image>) -> PlayerBundle {
        let mut starting_bullets = Vec::new();
        starting_bullets.push(gun::BulletBlueprint(1,|_| 5., |x| 9.*(x*10.).cos(), 0., true, 50));
        starting_bullets.push(gun::BulletBlueprint(1,|_| 5., |x| -9.*(x*10.).cos(), 0., true, 50));
        starting_bullets.push(gun::BulletBlueprint(1,|_| 20., |_: f32| 0., 0., true, 50));
        
        let mut s_gun = shapes::ShapeGun::default();
        s_gun.add_bloop(ShapeBloop{ offset: (0., 50.), num_bullets: 50, t: shapes::ShapeType::Triangle, size_scale: (0.2, 0.6)});
        s_gun.add_bloop(ShapeBloop{ offset: (55., -50.), num_bullets: 20, t: shapes::ShapeType::Triangle, size_scale: (0.2, 0.6)});
        s_gun.add_bloop(ShapeBloop{ offset: (-55., 0.), num_bullets: 30, t: shapes::ShapeType::Triangle, size_scale: (0.2, 0.6)});

        s_gun.bullet = gun::BulletBlueprint(1, |y| y*y, |x| 5. * (x*5.).cos(), 0., true, 60);

        PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset,
                transform: Transform::from_xyz(SPAWN_X, SPAWN_Y, 1.),
                ..default()
            },
            control: PlayerControlled,
            health: health::Health::new(SHIELD_SIZE, HEALTH_SIZE, 3.75, 100),
            gun: gun::Gun::new(starting_bullets, SHOT_DELAY, BULLET_DAMAGE, 10, 100, 3.0),
            s_gun: s_gun
        }
    } 
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>){
    
    let asset = asset_server.load("player.png");
    
    commands.spawn(
        PlayerBundle::new(asset)
    );


    commands.spawn(EzTextBundle::new(String::from("Health: "), 60., 820., 20., asset_server.load("fonts/EvilEmpire.otf"), Color::RED,HealthText));
    commands.spawn(EzTextBundle::new(String::from("Shield: "), 60., 880., 20., asset_server.load("fonts/EvilEmpire.otf"), Color::TEAL,ShieldText));
    commands.spawn(EzTextBundle::new(String::from(""), 40., 40., 20., asset_server.load("fonts/EvilEmpire.otf"), Color::GOLD,ScoreText));
    commands.spawn(EzTextBundle::new(String::from("00:00"), 40., 40., 940., asset_server.load("fonts/EvilEmpire.otf"), Color::GOLD,TimeText));
    commands.spawn(EzTextBundle::new(String::from("00000/00000"), 40., 780., 20., asset_server.load("fonts/EvilEmpire.otf"), Color::GOLD,AmmoText));
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


pub fn update_time_display(
    mut query: Query<&mut Text, With<TimeText>>, 
    game_time: Res<game::GameTimer>
) {
    let elapsed_secs = game_time.0.elapsed_secs();
    let minutes = elapsed_secs as u64 / 60;
    let seconds = elapsed_secs as u64 % 60;
    for mut text in &mut query {
        text.sections[0].value = format!("{:02}:{:02}", minutes, seconds);
    }
}

pub fn update_ammo_display(
    query: Query<&gun::Gun, With<PlayerControlled>>,
    mut ammo: Query<&mut Text, With<AmmoText>>
){
    if let Ok(gun) = query.get_single() {
        for mut text in &mut ammo {
            text.sections[0].value = format!("{:05}/{:05}", gun.get_ammo(), gun.get_max_ammo());
        }
    }
}


