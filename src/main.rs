use bevy::app::*;
use bevy::DefaultPlugins;
use bevy::prelude::AppExtStates;
use crate::main_menu_plugin::main_menu_plugin::MainMenuPlugin;
use crate::app_states::AppState;

mod rts_plugin;
mod main_menu_plugin;
mod app_states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .init_state::<AppState>()
        .run();
}
fn hello_world() {
    println!("hello world!");
}
