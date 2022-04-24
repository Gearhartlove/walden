use bevy::prelude::*;
use card_plugin::CardPlugin;

fn main() {
    let mut app: App = App::new();
    app
        .add_startup_system(card_plugin::add_cards)
        .add_plugin(CardPlugin)
        .add_system(hello_bevy)
        .run();
}

fn hello_bevy() {
    println!("hello bevy!");
}