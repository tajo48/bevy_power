use bevy::prelude::*;

/// Main power bar component that tracks current and maximum power
#[derive(Component, Debug, Clone)]
pub struct PowerBar {
    /// Current power value
    pub current: f32,
    /// Maximum power value (can be reduced by limits)
    pub max: f32,
    /// Base maximum power (without limits)
    pub base_max: f32,
    /// Whether the player is knocked out
    pub is_knocked_out: bool,
}

impl Default for PowerBar {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            base_max: 100.0,
            is_knocked_out: false,
        }
    }
}

impl PowerBar {
    /// Create a new power bar with specified max power
    pub fn new(max_power: f32) -> Self {
        Self {
            current: max_power,
            max: max_power,
            base_max: max_power,
            is_knocked_out: false,
        }
    }

    /// Spend power, returns true if successful
    pub fn spend(&mut self, amount: f32) -> bool {
        if self.is_knocked_out || self.current < amount {
            return false;
        }
        self.current = (self.current - amount).max(0.0);
        true
    }

    /// Add power, clamped to max
    pub fn add(&mut self, amount: f32) {
        if !self.is_knocked_out {
            self.current = (self.current + amount).min(self.max);
        }
    }

    /// Revive from knocked out state
    pub fn revive(&mut self, power_amount: f32) {
        if self.is_knocked_out {
            self.is_knocked_out = false;
            self.current = power_amount.min(self.max);
        }
    }

    /// Get power percentage (0.0 to 1.0)
    pub fn percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

/// Tracks the power level for progression
#[derive(Component, Debug, Clone)]
pub struct PowerLevel {
    /// Current level
    pub level: u32,
    /// Experience or progression points
    pub experience: f32,
    /// Experience needed for next level
    pub experience_to_next: f32,
}

impl Default for PowerLevel {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0.0,
            experience_to_next: 100.0,
        }
    }
}

impl PowerLevel {
    /// Level up and calculate new max power bonus
    pub fn level_up(&mut self) -> f32 {
        self.level += 1;
        self.experience = 0.0;
        self.experience_to_next *= 1.5; // Increase exp requirement

        // Calculate power bonus - diminishing returns
        let bonus = 20.0 / (1.0 + (self.level as f32 - 1.0) * 0.2);
        bonus
    }

    /// Add experience and check for level up
    pub fn add_experience(&mut self, amount: f32) -> bool {
        self.experience += amount;
        if self.experience >= self.experience_to_next {
            return true;
        }
        false
    }
}

/// Handles power regeneration mechanics
#[derive(Component, Debug, Clone)]
pub struct PowerRegeneration {
    /// Time since last power spend
    pub time_since_spend: f32,
    /// Delay before regeneration starts
    pub regen_delay: f32,
    /// Current regeneration rate
    pub current_rate: f32,
    /// Base regeneration rate
    pub base_rate: f32,
    /// Maximum regeneration rate
    pub max_rate: f32,
    /// Ramp up speed
    pub ramp_speed: f32,
    /// Whether regeneration is active
    pub is_active: bool,
}

impl Default for PowerRegeneration {
    fn default() -> Self {
        Self {
            time_since_spend: 0.0,
            regen_delay: 2.5,
            current_rate: 0.0,
            base_rate: 5.0,
            max_rate: 20.0,
            ramp_speed: 2.0,
            is_active: false,
        }
    }
}

impl PowerRegeneration {
    /// Reset regeneration when power is spent
    pub fn reset(&mut self) {
        self.time_since_spend = 0.0;
        self.current_rate = 0.0;
        self.is_active = false;
    }

    /// Update regeneration state
    pub fn update(&mut self, delta: f32) {
        self.time_since_spend += delta;

        if self.time_since_spend >= self.regen_delay {
            self.is_active = true;
            // Ramp up regeneration rate
            if self.current_rate < self.max_rate {
                self.current_rate =
                    (self.current_rate + self.ramp_speed * delta).min(self.max_rate);
                if self.current_rate == 0.0 {
                    self.current_rate = self.base_rate;
                }
            }
        }
    }

    /// Get the current regeneration amount for this frame
    pub fn get_regen_amount(&self, delta: f32) -> f32 {
        if self.is_active {
            self.current_rate * delta
        } else {
            0.0
        }
    }
}
