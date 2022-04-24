mod components;
mod parsing;

use components::Name;
use components::*;
use bevy::prelude::*;
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {

    }
}


/// Generates code for user. Used in each of walden's plugins.
/// Requires an implementation from the user.
pub trait GenerateCode {
    fn generate() {

    }
}

impl GenerateCode for CardPlugin {

}

impl CardPlugin {
    pub fn define_rare_qualities(qualities: &str) {
        let vec_qualities: Vec<String> =
            qualities.split_whitespace()
                .map(|s| s.to_string())
                .collect();
        // now I need to look into code generation
        println!("{:?}", vec_qualities);
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
