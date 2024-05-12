use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, ecs::schedule::ScheduleLabel, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*, window::{PresentMode, WindowTheme}};




mod bullet;
mod enemy;
mod player;
mod game;
mod menu;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

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
        .init_state::<GameState>()
        .add_plugins(menu::menu_plugin)
        .add_plugins(game::BulletHellElite)
        .run();
}

