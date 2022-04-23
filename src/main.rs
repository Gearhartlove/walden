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

    }
}

// entity
#[derive(Component, Debug)]
pub struct Card;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
struct Power(usize);

#[derive(Component)]
struct Health(usize);

#[derive(Component)]
struct Description(String);

#[derive(Component)]
// idea: could make a custom parser for the ability of the
// card which would allow users to write their own cards
// and play with them
struct Ability(String); // make ability a closure // complicated behavior
struct Keywords(Vec<String>);

#[derive(Component)]
struct Rarity(RareQualities);
enum RareQualities {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Component)]
struct CardTypes;
// enum CardTypes {} // Sub-card types

#[derive(Component)]
// idea: write a ai generated card art which is inspired by
// the name and abilities of the card
struct Art(String); // string is a path to the card art

#[derive(Component)]
struct ResourceCost(usize);

fn print_cards_names(query: Query<&Name, With<Card>>) {
    for card in query.iter() {
        println!("{}", card.0)
    }
}

fn add_cards(mut commands: Commands) {
    commands.spawn_batch(vec![
        (
            Card,
            Name("Clump of Whumps".to_string()),
            Power(2),
            Health(2),
            Ability("When I'm Summoned, create a Mushroom Cloud in hand.".to_string()),
            Rarity(RareQualities::Common),
            ResourceCost(2),
        ),
        (   Card,
            Name("Leeroy Jenkins".to_string()),
            Power(6),
            Health(2),
            Ability("Summon two 1/1 Whealps for your opponent".to_string()),
            Rarity(RareQualities::Legendary),
            ResourceCost(5),
            Keywords(vec!("Charge".to_string(), "Battlecry".to_string())),
            Art("path to art on computer".to_string()),
        ),
        (
            Card,
            Name("Llanowar Elves".to_string()),
            Power(1),
            Health(1),
            Description("As patient and generous as life, as harsh
            and merciless as nature".to_string()),
        ),
    ]);
}