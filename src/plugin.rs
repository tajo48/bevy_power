use crate::{
    components::{PowerBar, PowerLevel, PowerRegeneration},
    events::*,
    limits::PowerLimits,
    systems::*,
    ui::{setup_power_ui, update_power_bar_ui},
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

/// Plugin for the power system
pub struct PowerSystemPlugin;

impl Plugin for PowerSystemPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<SpendPowerEvent>()
            .add_event::<PowerChangeEvent>()
            .add_event::<ApplyLimitEvent>()
            .add_event::<LiftLimitEvent>()
            .add_event::<KnockedOutEvent>()
            .add_event::<ReviveEvent>()
            .add_event::<LevelUpEvent>();

        // Configure system sets
        app.configure_sets(
            Update,
            (
                PowerSystemSet::Input,
                PowerSystemSet::Update,
                PowerSystemSet::UI,
            )
                .chain(),
        );

        // Add startup systems
        app.add_systems(Startup, setup_power_ui);

        // Add update systems in proper order
        app.add_systems(
            Update,
            (
                // Input/Event handling
                (
                    handle_spend_power,
                    handle_power_change,
                    handle_apply_limit,
                    handle_lift_limit,
                    handle_revive,
                )
                    .in_set(PowerSystemSet::Input),
                // Core updates
                (
                    regenerate_power,
                    update_limit_timers,
                    detect_knockout,
                    handle_level_up,
                )
                    .in_set(PowerSystemSet::Update),
                // UI updates
                update_power_bar_ui.in_set(PowerSystemSet::UI),
            ),
        );
    }
}

/// Bundle for spawning an entity with power components
#[derive(Bundle, Default)]
pub struct PowerBundle {
    pub power_bar: PowerBar,
    pub power_level: PowerLevel,
    pub power_regeneration: PowerRegeneration,
    pub power_limits: PowerLimits,
}

impl PowerBundle {
    /// Create a new power bundle with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a power bundle with custom max power
    pub fn with_max_power(max_power: f32) -> Self {
        Self {
            power_bar: PowerBar::new(max_power),
            ..Default::default()
        }
    }

    /// Create a power bundle with custom settings
    pub fn custom(
        max_power: f32,
        regen_delay: f32,
        base_regen_rate: f32,
        max_regen_rate: f32,
    ) -> Self {
        Self {
            power_bar: PowerBar::new(max_power),
            power_level: PowerLevel::default(),
            power_regeneration: PowerRegeneration {
                regen_delay,
                base_rate: base_regen_rate,
                max_rate: max_regen_rate,
                ramp_speed: 2.0,
                ..Default::default()
            },
            power_limits: PowerLimits::default(),
        }
    }
}

/// Helper trait for easier power system interaction
pub trait PowerSystemExt {
    /// Spend power from this entity
    fn spend_power(&self, entity: Entity, amount: f32);

    /// Add power to this entity
    fn add_power(&self, entity: Entity, amount: f32);

    /// Apply a power limit
    fn apply_limit(
        &self,
        entity: Entity,
        id: u32,
        limit_type: crate::limits::LimitType,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    );

    /// Remove a power limit
    fn lift_limit(&self, entity: Entity, id: u32);

    /// Revive a knocked out entity
    fn revive(&self, entity: Entity, power_amount: f32);
}

impl PowerSystemExt for EventWriter<'_, SpendPowerEvent> {
    fn spend_power(&self, _entity: Entity, _amount: f32) {
        // This would need to be implemented differently
        // as we can't send events from a trait impl directly
        // This is more for demonstration of the API
    }

    fn add_power(&self, _entity: Entity, _amount: f32) {}
    fn apply_limit(
        &self,
        _entity: Entity,
        _id: u32,
        _limit_type: crate::limits::LimitType,
        _color: Color,
        _duration: Option<f32>,
        _resets_cooldown: bool,
    ) {
    }
    fn lift_limit(&self, _entity: Entity, _id: u32) {}
    fn revive(&self, _entity: Entity, _power_amount: f32) {}
}

/// System parameters for convenient power system access
#[derive(SystemParam)]
pub struct PowerSystem<'w> {
    pub spend_events: EventWriter<'w, SpendPowerEvent>,
    pub change_events: EventWriter<'w, PowerChangeEvent>,
    pub limit_events: EventWriter<'w, ApplyLimitEvent>,
    pub lift_events: EventWriter<'w, LiftLimitEvent>,
    pub revive_events: EventWriter<'w, ReviveEvent>,
}

impl<'w> PowerSystem<'w> {
    /// Spend power from an entity
    pub fn spend(&mut self, entity: Entity, amount: f32) {
        self.spend_events.write(SpendPowerEvent { entity, amount });
    }

    /// Change power (add or subtract)
    pub fn change(&mut self, entity: Entity, amount: f32) {
        self.change_events
            .write(PowerChangeEvent { entity, amount });
    }

    /// Apply a points-based limit
    pub fn limit_points(
        &mut self,
        entity: Entity,
        id: u32,
        points: f32,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) {
        self.limit_events.write(ApplyLimitEvent::points(
            entity,
            id,
            points,
            color,
            duration,
            resets_cooldown,
        ));
    }

    /// Apply a percentage-based limit
    pub fn limit_percentage(
        &mut self,
        entity: Entity,
        id: u32,
        percentage: f32,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) {
        self.limit_events.write(ApplyLimitEvent::percentage(
            entity,
            id,
            percentage,
            color,
            duration,
            resets_cooldown,
        ));
    }

    /// Lift a limit from an entity
    pub fn lift(&mut self, entity: Entity, limit_id: u32) {
        self.lift_events.write(LiftLimitEvent {
            entity,
            id: limit_id,
        });
    }

    /// Revive a knocked out entity
    pub fn revive(&mut self, entity: Entity, power_amount: f32) {
        self.revive_events.write(ReviveEvent {
            entity,
            power_amount,
        });
    }
}
