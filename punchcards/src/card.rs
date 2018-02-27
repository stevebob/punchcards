use append::Append;
use entity_store::*;
use animation::*;
use direction::CardinalDirection;
use common_animations;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Card {
    Move,
    Punch,
}

impl Card {
    pub fn play<Changes, Reactions>(
        self,
        entity_id: EntityId,
        entity_store: &EntityStore,
        direction: CardinalDirection,
        id_allocator: &mut EntityIdAllocator,
        changes: &mut Changes,
        reactions: &mut Reactions,
    ) where
        Changes: Append<EntityChange>,
        Reactions: Append<Animation>,
    {
        match self {
            Card::Move => {
                let current = entity_store.coord.get(&entity_id).unwrap();
                let delta = direction.coord();
                let new = *current + delta;
                changes.append(insert::coord(entity_id, new));
            }
            Card::Punch => {
                let source_coord = entity_store.coord.get(&entity_id).unwrap();
                let delta = direction.vector();
                let coord = *source_coord + delta;
                let punch_id = id_allocator.allocate();

                common_animations::punch(punch_id, coord, direction, reactions);
            }
        }
    }
}
