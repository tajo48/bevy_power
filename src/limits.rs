use bevy::prelude::*;

/// Type of power limit - either fixed points or percentage
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LimitType {
    /// Fixed amount of power points
    Points(f32),
    /// Percentage of base max power
    Percentage(f32),
}

/// Represents a power limit that reduces available power
#[derive(Component, Debug, Clone)]
pub struct PowerLimit {
    /// Unique identifier for this limit
    pub id: u32,
    /// Type and amount of limit
    pub limit_type: LimitType,
    /// Color for UI display
    pub color: Color,
    /// Optional timer for auto-removal (None = permanent until event)
    pub duration: Option<Timer>,
    /// Whether this limit resets the 2.5s cooldown when applied
    pub resets_cooldown: bool,
    /// Actual power value this limit takes
    pub power_value: f32,
}

impl PowerLimit {
    /// Create a new power limit
    pub fn new(
        id: u32,
        limit_type: LimitType,
        color: Color,
        duration: Option<f32>,
        resets_cooldown: bool,
    ) -> Self {
        Self {
            id,
            limit_type,
            color,
            duration: duration.map(|d| Timer::from_seconds(d, TimerMode::Once)),
            resets_cooldown,
            power_value: 0.0,
        }
    }

    /// Calculate the actual power value based on base max
    pub fn calculate_value(&mut self, base_max: f32) {
        self.power_value = match self.limit_type {
            LimitType::Points(points) => points,
            LimitType::Percentage(percent) => base_max * (percent / 100.0),
        };
    }

    /// Update timer and check if limit should be removed
    pub fn update(&mut self, delta: f32) -> bool {
        if let Some(ref mut timer) = self.duration {
            timer.tick(std::time::Duration::from_secs_f32(delta));
            timer.finished()
        } else {
            false
        }
    }

    /// Check if this limit is permanent (no timer)
    pub fn is_permanent(&self) -> bool {
        self.duration.is_none()
    }
}

/// Bundle of active power limits
#[derive(Component, Default, Debug)]
pub struct PowerLimits {
    pub limits: Vec<PowerLimit>,
}

impl PowerLimits {
    /// Add a new limit
    pub fn add_limit(&mut self, mut limit: PowerLimit, base_max: f32) {
        limit.calculate_value(base_max);
        self.limits.push(limit);
    }

    /// Remove a limit by ID
    pub fn remove_limit(&mut self, id: u32) -> bool {
        if let Some(index) = self.limits.iter().position(|l| l.id == id) {
            self.limits.remove(index);
            true
        } else {
            false
        }
    }

    /// Get total power reduction from all limits
    pub fn total_reduction(&self) -> f32 {
        self.limits.iter().map(|l| l.power_value).sum()
    }

    /// Update all limit timers and remove expired ones
    pub fn update_timers(&mut self, delta: f32) -> Vec<u32> {
        let mut removed_ids = Vec::new();

        self.limits.retain_mut(|limit| {
            if limit.update(delta) {
                removed_ids.push(limit.id);
                false
            } else {
                true
            }
        });

        removed_ids
    }

    /// Check if any limit resets cooldown
    pub fn any_resets_cooldown(&self) -> bool {
        self.limits.iter().any(|l| l.resets_cooldown)
    }

    /// Get all limit colors and their percentages for UI rendering
    pub fn get_limit_segments(&self, total_max: f32) -> Vec<(Color, f32)> {
        self.limits
            .iter()
            .map(|l| {
                let percentage = if total_max > 0.0 {
                    l.power_value / total_max
                } else {
                    0.0
                };
                (l.color, percentage)
            })
            .collect()
    }
}
