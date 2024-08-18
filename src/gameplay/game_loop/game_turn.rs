use crate::prelude::*;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(InGameplay = InGameplay)]
pub enum GameTurn {
    #[default]
    Setup,
    PlayerTurn,
    EnemyTurn,
    Waiting {
        next_state: Box<GameTurn>,
    },
    Encounter,
}

impl GameTurn {
    pub fn flip(&self) -> Self {
        match *self {
            GameTurn::PlayerTurn => GameTurn::EnemyTurn,
            GameTurn::EnemyTurn => GameTurn::PlayerTurn,
            _ => panic!("can flip turn only between Player<->Enemy! can't flip {:?}", *self),
        }
    }

    pub fn is_waiting() -> IsWaiting {
        IsWaiting
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct IsWaiting;

impl ComputedStates for IsWaiting {
    type SourceStates = GameTurn;

    fn compute(sources: GameTurn) -> Option<Self> {
        match sources {
            GameTurn::Waiting { .. } => Some(Self),
            _ => None,
        }
    }
}
