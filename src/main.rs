use bevy::prelude::*;
use card_plugin::CardPlugin;

fn main() {
    CardPlugin::define_rare_qualities("Rare Common BROKEN");
    // idea: chain card types together to create custom combos
    // CardPlugin::define_card_types("Creature 'Fast Spell' Sorcery");
    // idea: generate trait for my plugins for them to generate code
    // CardPlugin::generate_code();

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

