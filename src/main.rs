use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*};

const MOVE_SPEED: f32 = 125.;
const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;
const BULLET_DEATH: f32 = 5.;
const SPAWN_Y: f32 = -200.;
const SPAWN_X: f32 = 0.;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, bullet_movement)
        .run();
}

#[derive(Component)]
/// Player information / tag
struct PlayerControlled;

#[derive(Component)]
/// AI Information
struct AIControlled {
    t: EnemyType,   
}

/// Enemy type enum to determine movement / combat patterns
enum EnemyType {
    Melee, // Chase the player attempt to kamakazi them
    Linear, // shoot a straight shot either directly infront or at the player (maybe even fucking random)
    Wavy, // Shoot some cos/sin variant shot 
    Spammer, // shoot massive amounts of shit 
    Spawner // Shoot some bursts but primarily spawn more of the other types of enemies when killed spawn 2 spawners lol (consequences)
}

#[derive(Bundle)]
struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    ai_type: AIControlled
}

impl EnemyBundle {
    
}

#[derive(Component)]
/// Bullet Struct ;)
struct Bullet
{
    dir: i8,
    fx: fn(f32) -> f32,
    fy: fn(f32) -> f32,
    tick: f32

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
        mut sprite_position: Query<(&mut PlayerControlled, &mut Transform)>,
        keycode: Res<ButtonInput<KeyCode>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
    for (_, mut transform) in &mut sprite_position {
       
        // Constrain to bounds of screen
        if transform.translation.x > R_BOUND as f32 {
            transform.translation.x -= 50.;
        } else if transform.translation.x < -(L_BOUND as f32) {
            transform.translation.x += 50.;
        }

        let speed_mult = if keycode.pressed(KeyCode::ShiftLeft){ 2. } else { 1.}; // Speed boost

        //Move Left
        if keycode.pressed(KeyCode::KeyA) || keycode.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= MOVE_SPEED * time.delta_seconds() * speed_mult
        }
        // Move Right
        if keycode.pressed(KeyCode::KeyD) || keycode.pressed(KeyCode::ArrowRight) {
            transform.translation.x += MOVE_SPEED * time.delta_seconds() * speed_mult
        }

       
        // Shoot 
        if keycode.just_pressed(KeyCode::Space) {
            commands.spawn(BulletBundle::new(transform.translation.x, transform.translation.y, Bullet {dir: 1, fy: |_| 3., fx: |a: f32| 10.*(a).cos()  , tick: 0.}, asset_server.load("rocket.png")));
            commands.spawn( BulletBundle::new(transform.translation.x, transform.translation.y, Bullet {dir: 1, fy: |_| 50., fx: |_| 0. , tick: 0.}, asset_server.load("rocket.png")));
        }

        
    }
}

/// Move the bullets 
fn bullet_movement(
    time: Res<Time>, 
    mut sprite_position: Query<(Entity, &mut Bullet, &mut Transform)>,
    mut commands: Commands,
) {
    for (e, mut bullet,  mut transform) in &mut sprite_position {
        if bullet.tick > BULLET_DEATH  {
            commands.entity(e).despawn();
        }

        bullet.update(time.delta_seconds());
        
        transform.translation.y += (bullet.fy)(bullet.tick) * bullet.dir as f32; // run the y function
        transform.translation.x += (bullet.fx)(bullet.tick) * bullet.dir as f32; // run the x function

    }
}

