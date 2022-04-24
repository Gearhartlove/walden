use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Card;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Power(pub usize);

#[derive(Component)]
pub struct Health(pub usize);

#[derive(Component)]
pub struct Description(pub String);

#[derive(Component)]
// idea: could make a custom parser for the ability of the
// card which would allow users to write their own cards
// and play with them
pub struct Ability(pub String); // make ability a closure // complicated behavior
pub struct Keywords(Vec<String>);

#[derive(Component)]
pub struct Rarity(pub RareQualities);
pub enum RareQualities {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Component)]
pub struct CardTypes;
// enum CardTypes {} // Sub-card types

#[derive(Component)]
// idea: write a ai generated card art which is inspired by
// the name and abilities of the card
pub struct Art(pub String); // string is a path to the card art

#[derive(Component)]
pub struct ResourceCost(pub usize);
