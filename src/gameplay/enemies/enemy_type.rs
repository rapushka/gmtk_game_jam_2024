use crate::prelude::*;
use crate::gameplay::cards::types::CardType;

#[derive(Copy, Clone)]
pub enum EnemyType {
    Rat,
}

impl EnemyType {
    pub fn name(&self) -> String {
        match self {
            EnemyType::Rat => "rat",
        }.to_string()
    }

    pub fn cards(&self) -> Vec<CardType> {
        match self {
            EnemyType::Rat => vec![CardType::Attack(balance::RAT_ATTACK_COEFFICIENT)]
        }
    }
}