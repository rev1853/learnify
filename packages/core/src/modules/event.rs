use cosmwasm_std::{Attribute, Event};

pub fn get_wasm_event<'a>(events: &'a Vec<Event>, tag: &str) -> &'a Event {
    return get_event(events, &("wasm-".to_owned() + &*tag.to_owned()));
}

pub fn get_event<'a>(events: &'a Vec<Event>, tag: &str) -> &'a Event {
    return events.iter().find(|el| el.ty == tag.to_owned()).unwrap();
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
        return self.attributes.iter().find(|el| el.key == key).cloned();
    }

    fn get_attributes(&self, key: &str) -> Vec<Attribute> {
        return self
            .attributes
            .iter()
            .filter(|el| el.key == key)
            .cloned()
            .collect();
    }
}
