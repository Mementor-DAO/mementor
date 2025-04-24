use std::cell::RefCell;
use ic_stable_structures::Vec;
use crate::{
    memory::{get_events_memory, Memory}, 
    types::event::Event
};

pub struct EventStorage;

thread_local! {
    static EVENTS: RefCell<Vec<Event, Memory>> = RefCell::new(
        Vec::init(
            get_events_memory()
        ).unwrap()
    );
}

impl EventStorage {
    pub fn save(
        event: Event
    ) {
        EVENTS.with_borrow_mut(|events| {
            events.push(&event)
        }).unwrap();
    }

    pub fn slice(
        offset: usize,
        size: usize
    ) -> std::vec::Vec<Event> {
        let mut slice = vec![];

        EVENTS.with_borrow(|events| {
            for event in events.iter()
                .skip(offset)
                .take(size) {
                slice.push(event);
            }
        });

        slice
    }
}