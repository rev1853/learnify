use crate::state::config::Config;
use cw_storage_plus::Item;

pub struct State<'a> {
    pub config: Item<'a, Config>,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        return Self {
            config: Item::new("CONFIG"),
        };
    }
}
