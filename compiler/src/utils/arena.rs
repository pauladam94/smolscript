use id_arena::ArenaBehavior;

pub struct CompactArenaBehavior {}

impl ArenaBehavior for CompactArenaBehavior {
    type Id = u16;

    fn new_id(arena_id: u32, index: usize) -> Self::Id {
        index as u16
    }

    fn index(id: Self::Id) -> usize {
        id as usize
    }

    fn arena_id(id: Self::Id) -> u32 {
        0
    }
}

pub type Arena<T> = id_arena::Arena<T, CompactArenaBehavior>;
pub type ArenaID = u32;
