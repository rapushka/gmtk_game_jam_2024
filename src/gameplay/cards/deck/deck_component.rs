use std::collections::VecDeque;
use crate::prelude::{Component, Entity};

type Queue<T> = VecDeque<T>;

#[derive(Component)]
pub struct Deck {
    cards: Queue<Entity>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: Queue::<Entity>::new() }
    }

    pub fn add_card(&mut self, entity: Entity) {
        self.cards.push_back(entity);
    }

    pub fn card_count(&self) -> u8 {
        self.cards.len() as u8
    }

    /// returns the first card in the queue and puts it in the end of the queue
    pub fn pick_top_card(&mut self) -> Option<Entity> {
        let top_card = self.cards.pop_front()?;
        self.add_card(top_card);

        Some(top_card)
    }

    pub fn clone_cards(&self) -> Queue<Entity> {
        self.cards.clone()
    }
}