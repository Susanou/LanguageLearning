use bevy::prelude::*;
use bevy::log;

use crate::resources::tile;
use crate::{Board, Bomb, BombNeighbor, Coordinates, Uncover};
use crate::events::TileTriggerEvent;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_evr.iter() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    for (entity, parent) in children.iter() {
        commands
            .entity(entity)
            .despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(parent.0) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };

        // Try and remove the entity from the board's covered tiles hashmap
        match board.try_uncover_tile(coords) {
            None => log::debug!("Tried to uncover a tile already removed"),
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
        }

        if bomb.is_some() {
            log::info!("Explosion!");
            // TODO: add explosion and game over event
        }
        else if bomb_counter.is_none() {
            // When the tile is empty, we propagate the event to other neighbors
            // We add the `Uncover` component which will remove the tile on the next frame
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}