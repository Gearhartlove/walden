use bevy::prelude::*;

fn main() {
    let mut app: App = App::new();
    app
        .add_startup_system(add_cards)
        .add_plugin(CardPlugin)
        .add_system(print_cards_names)
        .run();
}

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(hello_world);
    }
}

fn hello_world() {
    println!("hello walden!")
}

// entity
#[derive(Component, Debug)]
pub struct Card(String);

#[derive(Component)]
struct HasPower(usize);

#[derive(Component)]
struct HasHealth(usize);

//system
// fn print_cards(query: Query<&Card, With<Information>>) {
//     for q in query.iter() {
//         println!("card: {:?}", q)
//     }
// }
fn print_cards_names(query: Query<&Card>) {
    for card in query.iter() {
        println!("{}", card.0)
    }
}

//////////
fn add_cards(mut commands: Commands) {
    commands.spawn_batch(vec![
        (
            Card("Phoenix".to_string()),
            HasPower(7),
            HasHealth(3),
        ),
        (
            Card("Turtle".to_string()),
            HasPower(2),
            HasHealth(9),
        ),
    ]);
}