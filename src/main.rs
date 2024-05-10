use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*};

const MOVE_SPEED: f32 = 125.;
const L_BOUND: u16 = 500;
const R_BOUND: u16 = 500;
const BULLET_DEATH: f32 = 20.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, bullet_movement)
        .run();
}

#[derive(Component)]
struct PlayerControlled;

#[derive(Component)]
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


    commands.spawn(( // Dumpster sprite
        SpriteBundle {
            texture: asset_server.load("jet.png"),
            transform: Transform::from_xyz(100., -100., 0.),
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
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("rocket.png"),
                    transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.),
                    ..default()
                },
                Bullet {dir: 1, fy: |a| 2.*a + 5., fx: |a: f32| 10.*(a*5.).cos() , tick: 0.},
            ));
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("rocket.png"),
                    transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.),
                    ..default()
                },
                Bullet {dir: 1, fy: |a| 10.*(a*5.).sin(), fx: |a: f32| 10.*(a*5.).cos() , tick: 0.},
            ));
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
        
        transform.translation.y += (bullet.fy)(bullet.tick) * bullet.dir as f32; //
        transform.translation.x += (bullet.fx)(bullet.tick) * bullet.dir as f32;

    }
}

