use crate::{
    bundles,
    component::{Name, *},
};
use bevy::prelude::*;

pub fn spawn_entities(mut commands: Commands) {
    commands.spawn(bundles::PlayerBundle::default());
    bundles::goblin(&mut commands, 3, 3);
    bundles::goblin(&mut commands, 10, 3);
    bundles::goblin(&mut commands, 3, 10);
    bundles::orc(&mut commands, 20, 20)
}

pub fn print_player(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("hello {}!", name.to_string())
    }
}
