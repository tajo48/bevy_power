use crate::limits::LimitType;
use bevy::prelude::*;

/// Event to spend power
#[derive(Event, Debug, Clone)]
pub struct SpendPowerEvent {
    /// Entity with the PowerBar component
    pub entity: Entity,
    /// Amount of power to spend
    pub amount: f32,
}

/// Event to change power (add or subtract)
#[derive(Event, Debug, Clone)]
pub struct PowerChangeEvent {
    /// Entity with the PowerBar component
    pub entity: Entity,
    /// Amount to change (negative for decrease)
    pub amount: f32,
}

/// Event to apply a power limit
#[derive(Event, Debug, Clone)]
pub struct ApplyLimitEvent {
    /// Entity to apply the limit to
    pub entity: Entity,
    /// Unique ID for this limit
    pub id: u32,
    /// Type of limit (points or percentage)
    pub limit_type: LimitType,
    /// Color for UI display
    pub color: Color,
    /// Duration in seconds (None for permanent)
    pub duration: Option<f32>,
    /// Whether this limit resets the regeneration cooldown
    pub resets_cooldown: bool,
}

impl ApplyLimitEvent {
    /// Create a new limit event with points
    pub fn points(
        entity: Entity,
        id: u32,
        points: f32,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) -> Self {
        Self {
            entity,
            id,
            limit_type: LimitType::Points(points),
            color,
            duration,
            resets_cooldown,
        }
    }

    /// Create a new limit event with percentage
    pub fn percentage(
        entity: Entity,
        id: u32,
        percentage: f32,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) -> Self {
        Self {
            entity,
            id,
            limit_type: LimitType::Percentage(percentage),
            color,
            duration,
            resets_cooldown,
        }
    }
}

/// Event to lift/remove a power limit
#[derive(Event, Debug, Clone)]
pub struct LiftLimitEvent {
    /// Entity to remove the limit from
    pub entity: Entity,
    /// ID of the limit to remove
    pub id: u32,
}

/// Event sent when player is knocked out
#[derive(Event, Debug, Clone)]
pub struct KnockedOutEvent {
    /// Entity that was knocked out
    pub entity: Entity,
}

/// Event to revive a knocked out player
#[derive(Event, Debug, Clone)]
pub struct ReviveEvent {
    /// Entity to revive
    pub entity: Entity,
    /// Amount of power to restore upon revival
    pub power_amount: f32,
}

/// Event sent when player levels up
#[derive(Event, Debug, Clone)]
pub struct LevelUpEvent {
    /// Entity that leveled up
    pub entity: Entity,
    /// New level
    pub new_level: u32,
    /// Power bonus gained
    pub power_bonus: f32,
}
