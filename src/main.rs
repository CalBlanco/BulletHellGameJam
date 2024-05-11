use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, ecs::schedule::ScheduleLabel, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*, window::{PresentMode, WindowTheme}};
use rand::Rng;
const MOVE_SPEED: f32 = 125.;
const PLY_MOVE_SPEED: f32 = 180.;
const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;
const T_BOUND: u16 = 300;
const B_BOUND: i16 = -200;
const BULLET_DEATH: f32 = 5.;
const SPAWN_Y: f32 = -200.;
const SPAWN_X: f32 = 0.;
const SHOOT_DELAY: f32 = 1.5;
const SPAWN_DELAY: f32 = 15.;
const JUMP_SIZE: f32 = 300.;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct SpawnUpdate;



fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1920., 1080.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, init_wave)
        .add_systems(FixedUpdate, sprite_movement)
        .add_systems(FixedUpdate, bullet_movement)
        .add_systems(FixedUpdate, enemy_control)
        .run();
}

#[derive(Component)]
/// Player information / tag
struct PlayerControlled;


#[derive(Component)]
/// Enemy type enum to determine movement / combat patterns
enum EnemyType {
    Melee, // Chase the player attempt to kamakazi them
    Linear, // shoot a straight shot either directly infront or at the player (maybe even fucking random)
    Wavy, // Shoot some cos/sin variant shot 
    Spammer, // shoot massive amounts of shit 
    Spawner, // Shoot some bursts but primarily spawn more of the other types of enemies when killed spawn 2 spawners lol (consequences)
}

#[derive(Component)]
struct Enemy {
    t: EnemyType,
    dir: i8,
    last_shot: f32
}


#[derive(Bundle)]
struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
    collider: Collider
}

/// Create a new enemey providing a spawn location, type and asset to render
impl EnemyBundle {
    fn new(spawn_x: f32, spawn_y: f32, t: EnemyType, asset: Handle<Image>) -> EnemyBundle{
        EnemyBundle {
            sprite_bundle: SpriteBundle {
                texture: asset,
                transform: Transform::from_xyz(spawn_x, spawn_y, 0.),
                ..default()
            },
            enemy: Enemy {
                dir: 1,
                t: t,
                last_shot: 0.
            },
            collider: Collider
        }
    }
}




#[derive(Component)]
/// Bullet Struct ;)
struct Bullet
{
    dir: i8,
    fx: fn(f32) -> f32,
    fy: fn(f32) -> f32,
    tick: f32,
    ply: bool

} // 

impl Bullet{
    /// Update the life time of the bullet 
    fn update(&mut self, time:f32) {
        self.tick += time;
    }
}

#[derive(Bundle)]
struct BulletBundle {
    sprite_bundle: SpriteBundle,
    bullet: Bullet,
}

/// Bundle to contain the bullet class
impl BulletBundle{
    /// Create a new bullet by passing a spawn position, bullet data (direction, fx, fy, and tick start), and a texture image from the Asset loader
    fn new(spawn_x: f32, spawn_y: f32, bullet: Bullet, asset: Handle<Image>) -> BulletBundle {
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


#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;


/// Setup our game world 
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(( //Camera with bloom settings enabled
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));


    commands.spawn(( // Player
        SpriteBundle {
            texture: asset_server.load("jet.png"),
            transform: Transform::from_xyz(SPAWN_X, SPAWN_Y, 0.),
            ..default()
        },
        PlayerControlled,
    ));

    
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
        time: Res<Time>, 
        mut sprite_position: Query<&mut Transform, With<PlayerControlled>>,
        keycode: Res<ButtonInput<KeyCode>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
    for mut transform in &mut sprite_position {
        
        // Constrain to bounds of screen
        if transform.translation.x > R_BOUND as f32 {
            transform.translation.x -= 50.;
        } else if transform.translation.x < -(L_BOUND as f32) {
            transform.translation.x += 50.;
        }

        let speed_mult = if keycode.pressed(KeyCode::ShiftLeft){ 3. } else { 1.}; // Speed boost

        //Move Left
        if keycode.pressed(KeyCode::KeyA) || keycode.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= MOVE_SPEED * time.delta_seconds() * speed_mult
        }
        // Move Right
        if keycode.pressed(KeyCode::KeyD) || keycode.pressed(KeyCode::ArrowRight) {
            transform.translation.x += MOVE_SPEED * time.delta_seconds() * speed_mult
        }

        if keycode.pressed(KeyCode::KeyQ) {
            let jump_x = if transform.translation.x - JUMP_SIZE < 0. - L_BOUND as f32 { 0. - L_BOUND as f32 } else {transform.translation.x - JUMP_SIZE};
            transform.translation.x = jump_x;
        }
        if keycode.pressed(KeyCode::KeyE) {
            let jump_x = if transform.translation.x + JUMP_SIZE > L_BOUND as f32 { L_BOUND as f32 } else {transform.translation.x + JUMP_SIZE};
            transform.translation.x = jump_x;
        }

       
        // Shoot 
        if keycode.just_pressed(KeyCode::Space) {
            commands.spawn(BulletBundle::new(transform.translation.x, transform.translation.y, Bullet {dir: 1, fy: |_| 3., fx: |a: f32| 10.*(a).cos()  , tick: 0., ply: true}, asset_server.load("rocket.png")));
            commands.spawn( BulletBundle::new(transform.translation.x, transform.translation.y, Bullet {dir: 1, fy: |_| 50., fx: |_| 0. , tick: 0., ply:true}, asset_server.load("rocket.png")));
        }

        
    }
}

/// Move the bullets 
fn bullet_movement(
    time: Res<Time>, 
    mut sprite_position: Query<(Entity, &mut Bullet, &mut Transform), (With<Bullet>, Without<Collider>)>,
    collider_query: Query<(Entity, &Transform), (With<Collider>, Without<Bullet>)>,
    player_query: Query<(Entity, &Transform), (With<PlayerControlled>, Without<Bullet>, Without<Collider>)>,
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
                        if col { commands.entity(p_ent).despawn(); } // despawn the enemy for now 
                        
                    }
                }
            }
            true => {
                // Check bullet collision with enemy
                for(collider_entity, e_transform) in &collider_query {
                    let collision = bullet_collision(Aabb2d::new(b_transform.translation.truncate(), b_transform.scale.truncate()/2.), Aabb2d::new(e_transform.translation.truncate(), Vec2::new(64.,64.)));
                    
                    if let Some(col) = collision { // collision between enemy and player bullet
                        if col { commands.entity(collider_entity).despawn(); } // despawn the enemy for now 
                        
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

/// Control enemy movement and behavior 
fn enemy_control(
    time: Res<Time>,
    mut sprite_position: Query<(Entity, &mut Transform, &mut Enemy)>,
    mut commands: Commands,
    mut asset_server: Res<AssetServer>

) {
    for(e, mut transform, mut enemy) in &mut sprite_position{
        if transform.translation.x <  -(L_BOUND as f32) || transform.translation.x > (R_BOUND as f32) {
            enemy.dir = enemy.dir * -1;
            transform.translation.y -= 96.;
        }

        transform.translation.x += MOVE_SPEED * time.delta_seconds() * enemy.dir as f32;

        
        enemy.last_shot += time.delta_seconds();

        if enemy.last_shot > SHOOT_DELAY{
            enemy.last_shot = 0.;
            let spawn_x = transform.translation.x;
            let spawn_y = transform.translation.y - 30.;
            match  enemy.t {
                EnemyType::Melee => {},
                EnemyType::Linear => { // |args| expr == fn(args) {expr}
                    commands.spawn(BulletBundle::new(spawn_x, spawn_y, Bullet {dir: -1, fy: |_| 20., fx: |_| 0., tick: 0., ply: false}, asset_server.load("rocket.png")));
                },
                EnemyType::Wavy => {
                    commands.spawn(BulletBundle::new(spawn_x, spawn_y, Bullet {dir: -1, fy: |_| 4., fx: |a| 10.*a.cos(), tick: 0., ply: false}, asset_server.load("rocket.png")));
                },
                EnemyType::Spammer => {
                    commands.spawn(BulletBundle::new(spawn_x, spawn_y, Bullet {dir: -1, fy: |a| 20.*a, fx: |_| 5., tick: 0., ply:false}, asset_server.load("rocket.png")));
                    commands.spawn(BulletBundle::new(spawn_x, spawn_y, Bullet {dir: -1, fy: |a| 20.*a, fx: |_| -5., tick: 0., ply:false}, asset_server.load("rocket.png")));
                    commands.spawn(BulletBundle::new(spawn_x, spawn_y, Bullet {dir: -1, fy: |_| 4., fx: |a| 10.*a.cos(), tick: 0., ply:false}, asset_server.load("rocket.png")));
                
                },
                EnemyType::Spawner => {
                    let rng_x = rand::thread_rng().gen_range(0..=5);
                    let rng_y = rand::thread_rng().gen_range(0..=5);
                    spawn_wave_box(rng_x, rng_y, &mut asset_server, &mut commands);
                    enemy.last_shot -= SPAWN_DELAY; // Set the spawn timer to have a larger delay than the shoot timer
                }
    
            }
        }
        
    }
}


fn spawn_wave_box(wave_rows: u32, wave_cols: u32, asset_server: &mut Res<AssetServer>, commands: &mut Commands) {
    for x in 0..wave_rows {
        for y in 0..wave_cols {
            
            let spawn_x =  (64. * x as f32 + 32.) - L_BOUND as f32;
            let spawn_y = T_BOUND as f32 + (64 * y) as f32;

            let rng = rand::thread_rng().gen_range(0..=100);
            match rng {
                0..=20 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Melee, asset_server.load("dump.png")));},
                21..=40 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Linear, asset_server.load("dump.png")));},
                41..=60 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Wavy, asset_server.load("dump.png")));},
                61..=80 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Spammer, asset_server.load("dump.png")));},
                81..=100 => {commands.spawn(EnemyBundle::new(spawn_x, spawn_y, EnemyType::Spawner, asset_server.load("dump.png")));},
                _ => ()
            }
        }
    }
}



fn init_wave(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>
){
        spawn_wave_box(5, 3, &mut asset_server, &mut commands)
}
