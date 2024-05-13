use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, window::{PresentMode, WindowTheme}};

pub const T_BOUND: u16 = 400;
pub const B_BOUND: f32 = -500.;
pub const PLAYBACK_SPEED: f32 = 2.0;

mod bullet;
mod enemy;
mod player;
mod game;
mod menu;
mod music;

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

#[derive(Bundle)]
pub struct EzTextBundle<T: Component> {
    text: TextBundle,
    tag: T,
}

impl<T: Component> EzTextBundle<T> {
    pub fn new(text: String, size: f32, top: f32, left: f32, font: Handle<Font>, color: Color, tag: T ) -> EzTextBundle<T> {
        EzTextBundle {
            text: TextBundle::from_section(
                text,
                TextStyle {
                    font: font,
                    font_size: size,
                    color: color,
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(top),
                right: Val::Px(left),
                ..default()
            }),
            tag: tag
        }
    }
}