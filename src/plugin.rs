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

/// System parameters for convenient power system access
#[derive(SystemParam)]
pub struct PowerSystem<'w, 's> {
    pub spend_events: EventWriter<'w, SpendPowerEvent>,
    pub change_events: EventWriter<'w, PowerChangeEvent>,
    pub limit_events: EventWriter<'w, ApplyLimitEvent>,
    pub lift_events: EventWriter<'w, LiftLimitEvent>,
    pub revive_events: EventWriter<'w, ReviveEvent>,
    pub power_query: Query<'w, 's, (Entity, &'static mut PowerBar, Option<&'static PowerLimits>)>,
}

impl<'w, 's> PowerSystem<'w, 's> {
    /// Get the entity with PowerBar component (assumes single entity)
    fn get_power_entity(&self) -> Option<Entity> {
        self.power_query.iter().next().map(|(entity, _, _)| entity)
    }

    /// Check if the power entity can afford to spend the specified amount of power
    pub fn can_afford(&self, amount: f32) -> bool {
        if let Some(entity) = self.get_power_entity() {
            if let Ok((_, power_bar, _)) = self.power_query.get(entity) {
                return !power_bar.is_knocked_out && power_bar.current > amount;
            }
        }
        false
    }

    /// Try to spend power, returns true if successful
    pub fn try_spend(&mut self, amount: f32) -> bool {
        if let Some(entity) = self.get_power_entity() {
            if self.can_afford(amount) {
                self.spend_events.write(SpendPowerEvent { entity, amount });
                return true;
            }
        }
        false
    }

    /// Try to apply a points-based limit, returns true if successful
    pub fn try_limit_points(
        &mut self,
        id: u32,
        points: f32,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) -> bool {
        if let Some(entity) = self.get_power_entity() {
            // Check if applying this limit would cause a knockout
            if let Ok((_, power_bar, limits)) = self.power_query.get(entity) {
                let total_current_reduction = limits.map(|l| l.total_reduction()).unwrap_or(0.0);
                let new_total_reduction = total_current_reduction + points;
                let new_max = (power_bar.base_max - new_total_reduction).max(0.0);
                let new_current = power_bar.current.min(new_max);

                // Only apply if it won't cause knockout (max > 0 and current > 0)
                if new_max > 0.0 && new_current > 0.0 {
                    self.limit_events.write(ApplyLimitEvent::points(
                        entity,
                        id,
                        points,
                        color,
                        duration,
                        resets_cooldown,
                    ));
                    return true;
                }
            }
        }
        false
    }

    /// Try to apply a percentage-based limit, returns true if successful
    pub fn try_limit_percentage(
        &mut self,
        id: u32,
        percentage: f32,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) -> bool {
        if let Some(entity) = self.get_power_entity() {
            // Check if applying this limit would cause a knockout
            if let Ok((_, power_bar, limits)) = self.power_query.get(entity) {
                let percentage_points = power_bar.base_max * (percentage / 100.0);
                let total_current_reduction = limits.map(|l| l.total_reduction()).unwrap_or(0.0);
                let new_total_reduction = total_current_reduction + percentage_points;
                let new_max = (power_bar.base_max - new_total_reduction).max(0.0);
                let new_current = power_bar.current.min(new_max);

                // Only apply if it won't cause knockout (max > 0 and current > 0)
                if new_max > 0.0 && new_current > 0.0 {
                    self.limit_events.write(ApplyLimitEvent::percentage(
                        entity,
                        id,
                        percentage,
                        color,
                        duration,
                        resets_cooldown,
                    ));
                    return true;
                }
            }
        }
        false
    }

    /// Spend power (always sends event, may fail)
    pub fn spend(&mut self, amount: f32) {
        if let Some(entity) = self.get_power_entity() {
            self.spend_events.write(SpendPowerEvent { entity, amount });
        }
    }

    /// Change power (add or subtract)
    pub fn change(&mut self, amount: f32) {
        if let Some(entity) = self.get_power_entity() {
            self.change_events
                .write(PowerChangeEvent { entity, amount });
        }
    }

    /// Apply a points-based limit
    pub fn limit_points(
        &mut self,
        id: u32,
        points: f32,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) {
        if let Some(entity) = self.get_power_entity() {
            self.limit_events.write(ApplyLimitEvent::points(
                entity,
                id,
                points,
                color,
                duration,
                resets_cooldown,
            ));
        }
    }

    /// Apply a percentage-based limit
    pub fn limit_percentage(
        &mut self,
        id: u32,
        percentage: f32,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) {
        if let Some(entity) = self.get_power_entity() {
            self.limit_events.write(ApplyLimitEvent::percentage(
                entity,
                id,
                percentage,
                color,
                duration,
                resets_cooldown,
            ));
        }
    }

    /// Lift a limit
    pub fn lift(&mut self, limit_id: u32) {
        if let Some(entity) = self.get_power_entity() {
            self.lift_events.write(LiftLimitEvent {
                entity,
                id: limit_id,
            });
        }
    }

    /// Revive a knocked out entity
    pub fn revive(&mut self, power_amount: f32) {
        if let Some(entity) = self.get_power_entity() {
            self.revive_events.write(ReviveEvent {
                entity,
                power_amount,
            });
        }
    }
}
