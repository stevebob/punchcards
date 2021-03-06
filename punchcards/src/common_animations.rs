use std::time::Duration;
use grid_2d::Coord;
use direction::CardinalDirection;
use animation::*;
use entity_store::EntityId;
use prototypes::Prototype;
use timing;
use message_queues::PushMessages;

pub fn punch<M: PushMessages>(
    id: EntityId,
    coord: Coord,
    direction: CardinalDirection,
    messages: &mut M,
) {
    let punch = Prototype::Punch(id, coord, direction);
    temporary_at_coord(
        coord,
        punch,
        Duration::from_millis(timing::PUNCH_MILLIS),
        messages,
    );
}

pub fn bullet<M: PushMessages>(
    id: EntityId,
    messages: &mut M,
) {
    slide(id, Duration::from_millis(timing::BULLET_SLIDE_MILLIS), messages);
}
