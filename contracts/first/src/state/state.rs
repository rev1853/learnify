use cw_storage_plus::{IndexedMap, Item, Map};
use crate::state::citizen_info::CitizenInfo;
use crate::state::creator_info::{CreatorInfo};
use crate::state::song_info::{SongIndexes, SongInfo};

pub struct State<'a> {
    pub creator_info: Item<'a, CreatorInfo>,
    pub citizens: Map<'a, String, CitizenInfo>,
    pub songs: IndexedMap<'a, &'a str, SongInfo, SongIndexes<'a>>
}

impl<'a> State<'a>{
    pub fn new() -> Self {
        return Self {
            creator_info: Item::new("CREATOR_INFO"),
            citizens: Map::new("CITIZEN_INFO"),
            songs: IndexedMap::new("SONG", SongIndexes::new())
        }
    }
}