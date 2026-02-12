use bevy::app::*;

mod components;
mod systems;

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
fn hello_world() {
    println!("hello world!");
}
