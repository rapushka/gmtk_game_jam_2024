use crate::utils::constants;

#[derive(Copy, Clone)]
pub enum CardType {
    Attack(f32),
}

impl CardType {
    pub fn name(&self) -> String {
        match self {
            CardType::Attack(_) => "Attack".to_string(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            CardType::Attack(_) => format!("Deals Atk * {} Damage to Enemy", constants::balance::ATTACK_COEFFICIENT),
        }
    }
}