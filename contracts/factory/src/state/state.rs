use crate::state::config::Config;
use cw_storage_plus::{Item, Map};

pub struct State<'a> {
    pub config: Item<'a, Config>,
    pub pairs: Map<'a, String, String>,
    pub tokens: Map<'a, String, String>
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        return Self {
            config: Item::new("CONFIG"),
            pairs: Map::new("PAIRS"),
            tokens: Map::new("TOKENS")
        };
    }
}
