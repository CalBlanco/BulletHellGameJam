

use bevy::prelude::*;
use super::GameState;

#[derive(Component)]
struct Music;

pub fn make_plugin(app: &mut App){
    app
        .add_systems(Startup, setup_music)
        .add_systems(FixedUpdate, music_controls);
}



fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(
        (
            AudioBundle {
                source: asset_server.load("sounds/music/bigmusiclmao.ogg"),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Loop,
                    ..default()
                }
            },
            Music 
        )
    );
}

fn music_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink>
)
{

    
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        for sink in music_controller.iter() {
            sink.set_volume(sink.volume()  + 0.1);
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        for sink in music_controller.iter() {
            let vol = if sink.volume()  - 0.1 < 0. {0. } else {sink.volume()  - 0.1};
            sink.set_volume(vol);
        }
    }
    
}