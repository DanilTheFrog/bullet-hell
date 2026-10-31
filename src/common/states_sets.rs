use bevy::prelude::*;


#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameSet {
    UiInput,
    Input,
    Movement,
    Combat,
    Cleanup
}

#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CombatSet {
    TargetSelection,
    DamageCalculation,
    Cleanp
}

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameState {
    Paused,
    #[default]
    Running,
    Suspended
}

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameStage {
    Lobby,
    #[default]
    Level
}