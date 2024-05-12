use bevy::prelude::*;
use super::GameState;


pub fn make_plugin(app: &mut App){
    app
        .add_systems(Startup, setup_music)
        .add_systems(OnEnter(GameState::Game), setup_game_music)
        .add_systems(OnEnter(GameState::Menu), setup_menu_music)
        .add_systems(Update, play_music);
}

fn setup_music(){}
fn setup_game_music(){}
fn setup_menu_music(){}
fn play_music(){}