use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, ecs::schedule::ScheduleLabel, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*, window::{PresentMode, WindowTheme}};




mod player;
mod enemy;
mod bullet;



fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "BH: Elite".into(),
                    name: Some("BulletHellElite".into()),
                    resolution: (1040., 960.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: true,
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
        .add_event::<enemy::CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(Startup, player::spawn_player)
        .add_systems(Startup, enemy::init_wave)
        .add_systems(FixedUpdate, player::sprite_movement)
        .add_systems(FixedUpdate, bullet::bullet_movement)
        .add_systems(FixedUpdate, bullet::play_collision_sound)
        .add_systems(FixedUpdate, enemy::enemy_control)
        .run();
}


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

    

    
    
}
