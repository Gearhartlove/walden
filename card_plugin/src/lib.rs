mod components;

use components::Name;
use components::*;
use bevy::prelude::*;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {

    }
}


pub fn print_cards_names(query: Query<&Name, With<Card>>) {
    for card in query.iter() {
        println!("{}", card.0)
    }
}

pub fn add_cards(mut commands: Commands) {
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
        // (   Card,
        //     Name("Leeroy Jenkins".to_string()),
        //     Power(6),
        //     Health(2),
        //     Ability("Summon two 1/1 Whealps for your opponent".to_string()),
        //     Rarity(RareQualities::Legendary),
        //     ResourceCost(5),
        //     Keywords(vec!("Charge".to_string(), "Battlecry".to_string())),
        //     Art("path to art on computer".to_string()),
        // ),
        // (
        //     Card,
        //     Name("Llanowar Elves".to_string()),
        //     Power(1),
        //     Health(1),
        //     Description("As patient and generous as life, as harsh
        //     and merciless as nature".to_string()),
        // ),
    ]);
}
