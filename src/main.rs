use bevy::app::*;

mod rts_plugin;

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
fn hello_world() {
    println!("hello world!");
}
