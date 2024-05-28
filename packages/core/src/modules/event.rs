use cosmwasm_std::{Attribute, Event};

const MARKET_EVENT_TAG: &str = "bigbangx_market";
const NFT_EVENT_TAG: &str = "bigbangx_nft";
const COLLECTION_EVENT_TAG: &str = "bigbangx_collection";
const RENTING_EVENT_TAG: &str = "bigbangx_renting";

pub fn new_market_event() -> Event {
    return Event::new(MARKET_EVENT_TAG)
}

pub fn new_nft_event() -> Event {
    return Event::new(NFT_EVENT_TAG)
}

pub fn new_collection_event() -> Event {
    return Event::new(COLLECTION_EVENT_TAG)
}
pub fn new_renting_event() -> Event {
    return Event::new(RENTING_EVENT_TAG)
}

pub fn get_market_event(events: &Vec<Event>) -> & Event {
    return events.iter().find(|el| el.ty == "wasm-".to_owned() + MARKET_EVENT_TAG).unwrap();
}

pub fn get_nft_event(events: &Vec<Event>) -> & Event {
    return events.iter().find(|el| el.ty == "wasm-".to_owned() + NFT_EVENT_TAG).unwrap();
}

pub fn get_collection_event(events: &Vec<Event>) -> & Event {
    return events.iter().find(|el| el.ty == "wasm-".to_owned() + COLLECTION_EVENT_TAG).unwrap();
}

pub fn get_renting_event(events: &Vec<Event>) -> & Event {
    return events.iter().find(|el| el.ty == "wasm-".to_owned() + RENTING_EVENT_TAG).unwrap();
}

pub trait DefaultEvent {
    fn get_event(&self) -> Event;
}

pub trait AttributeHelper {
    fn get_attribute(&self, key: &str) -> Option<Attribute>;
    fn get_attributes(&self, key: &str) -> Vec<Attribute>;
}

impl AttributeHelper for Event {
    fn get_attribute(&self, key: &str) -> Option<Attribute> {
        return self.attributes.iter().find(|el| el.key == key).cloned()
    }

    fn get_attributes(&self, key: &str) -> Vec<Attribute> {
        return self.attributes.iter().filter(|el| el.key == key).cloned().collect()
    }
}