use crate::{
    components::{PowerBar, PowerLevel, PowerRegeneration},
    events::{
        ApplyLimitEvent, KnockedOutEvent, LevelUpEvent, LiftLimitEvent, PowerChangeEvent,
        ReviveEvent, SpendPowerEvent,
    },
    limits::{PowerLimit, PowerLimits},
};
use bevy::prelude::*;

/// System to handle power spending events
pub fn handle_spend_power(
    mut events: EventReader<SpendPowerEvent>,
    mut query: Query<(&mut PowerBar, &mut PowerRegeneration, Option<&PowerLimits>)>,
) {
    for event in events.read() {
        if let Ok((mut power_bar, mut regen, limits)) = query.get_mut(event.entity) {
            if power_bar.spend(event.amount) {
                // Reset regeneration on successful spend
                regen.reset();

                // Check if limits should reset cooldown
                if let Some(limits) = limits {
                    if limits.any_resets_cooldown() {
                        regen.reset();
                    }
                }
            }
        }
    }
}

/// System to handle power change events (add/subtract)
pub fn handle_power_change(
    mut events: EventReader<PowerChangeEvent>,
    mut query: Query<&mut PowerBar>,
) {
    for event in events.read() {
        if let Ok(mut power_bar) = query.get_mut(event.entity) {
            if event.amount > 0.0 {
                power_bar.add(event.amount);
            } else {
                power_bar.spend(event.amount.abs());
            }
        }
    }
}

/// System to handle power regeneration
pub fn regenerate_power(
    time: Res<Time>,
    mut query: Query<(&mut PowerBar, &mut PowerRegeneration, Option<&PowerLimits>)>,
) {
    let delta = time.delta_secs();

    for (mut power_bar, mut regen, limits) in query.iter_mut() {
        if !power_bar.is_knocked_out {
            // Check if any limits prevent regeneration
            let regeneration_blocked = limits.map(|l| l.any_stops_regeneration()).unwrap_or(false);

            if !regeneration_blocked {
                regen.update(delta);
                let regen_amount = regen.get_regen_amount(delta);
                if regen_amount > 0.0 {
                    power_bar.add(regen_amount);
                }
            }
        }
    }
}

/// System to handle applying power limits
pub fn handle_apply_limit(
    mut events: EventReader<ApplyLimitEvent>,
    mut query: Query<(&mut PowerBar, &mut PowerRegeneration, &mut PowerLimits)>,
    mut knocked_out_events: EventWriter<KnockedOutEvent>,
) {
    for event in events.read() {
        if let Ok((mut power_bar, mut regen, mut limits)) = query.get_mut(event.entity) {
            let new_limit = PowerLimit::new(
                event.id,
                event.limit_type,
                event.color,
                event.duration,
                event.resets_cooldown,
                event.stops_regeneration,
            );

            limits.add_limit(new_limit, power_bar.base_max);

            // Update max power based on limits
            let total_reduction = limits.total_reduction();
            power_bar.max = (power_bar.base_max - total_reduction).max(0.0);

            // Clamp current power to new max
            if power_bar.current > power_bar.max {
                power_bar.current = power_bar.max;
            }

            // Check for knockout
            if power_bar.max <= 0.0 || power_bar.current <= 0.0 {
                power_bar.is_knocked_out = true;
                knocked_out_events.write(KnockedOutEvent {
                    entity: event.entity,
                });
            }

            // Reset cooldown if needed
            if event.resets_cooldown {
                regen.reset();
            }
        }
    }
}

/// System to handle lifting power limits
pub fn handle_lift_limit(
    mut events: EventReader<LiftLimitEvent>,
    mut query: Query<(&mut PowerBar, &mut PowerLimits)>,
) {
    for event in events.read() {
        if let Ok((mut power_bar, mut limits)) = query.get_mut(event.entity) {
            if limits.remove_limit(event.id) {
                // Recalculate max power
                let total_reduction = limits.total_reduction();
                power_bar.max = (power_bar.base_max - total_reduction).max(0.0);

                // If knocked out but now has max power, allow revival
                if power_bar.is_knocked_out && power_bar.max > 0.0 {
                    power_bar.is_knocked_out = false;
                    power_bar.current = power_bar.current.min(power_bar.max);
                }
            }
        }
    }
}

/// System to update limit timers and remove expired ones
pub fn update_limit_timers(
    time: Res<Time>,
    mut query: Query<(Entity, &mut PowerBar, &mut PowerLimits)>,
) {
    let delta = time.delta_secs();

    for (_entity, mut power_bar, mut limits) in query.iter_mut() {
        let removed_ids = limits.update_timers(delta);

        // Update max power if any limits were removed
        if !removed_ids.is_empty() {
            let total_reduction = limits.total_reduction();
            power_bar.max = (power_bar.base_max - total_reduction).max(0.0);

            // If knocked out but now has max power, allow revival
            if power_bar.is_knocked_out && power_bar.max > 0.0 {
                power_bar.is_knocked_out = false;
                power_bar.current = power_bar.current.min(power_bar.max);
            }
        }
    }
}

/// System to handle revival events
pub fn handle_revive(mut events: EventReader<ReviveEvent>, mut query: Query<&mut PowerBar>) {
    for event in events.read() {
        if let Ok(mut power_bar) = query.get_mut(event.entity) {
            power_bar.revive(event.power_amount);
        }
    }
}

/// System to handle level up mechanics
pub fn handle_level_up(
    mut query: Query<(&mut PowerBar, &mut PowerLevel)>,
    mut level_up_events: EventWriter<LevelUpEvent>,
) {
    for (mut power_bar, mut power_level) in query.iter_mut() {
        // This would be triggered by game events adding experience
        // For demo purposes, we'll check if level up should occur
        if power_level.experience >= power_level.experience_to_next {
            let power_bonus = power_level.level_up();
            power_bar.base_max += power_bonus;
            power_bar.max += power_bonus;

            level_up_events.write(LevelUpEvent {
                entity: Entity::PLACEHOLDER,
                new_level: power_level.level,
                power_bonus,
            });
        }
    }
}

/// System to detect knockout conditions
pub fn detect_knockout(
    mut query: Query<(Entity, &mut PowerBar), Changed<PowerBar>>,
    mut knocked_out_events: EventWriter<KnockedOutEvent>,
) {
    for (entity, mut power_bar) in query.iter_mut() {
        if !power_bar.is_knocked_out && (power_bar.current <= 0.0 || power_bar.max <= 0.0) {
            power_bar.is_knocked_out = true;
            power_bar.current = 0.0;
            knocked_out_events.write(KnockedOutEvent { entity });
        }
    }
}

/// System set for power systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PowerSystemSet {
    /// Input handling and event processing
    Input,
    /// Core power system updates
    Update,
    /// UI updates
    UI,
}
