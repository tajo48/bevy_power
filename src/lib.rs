mod components;
mod events;
mod limits;
mod plugin;
mod systems;
mod ui;

pub use components::{PowerBar, PowerLevel, PowerRegeneration};
pub use events::{
    ApplyLimitEvent, KnockedOutEvent, LevelUpEvent, LiftLimitEvent, PowerChangeEvent, ReviveEvent,
    SpendPowerEvent,
};
pub use limits::{LimitType, PowerLimit, PowerLimits};
pub use plugin::{PowerBundle, PowerSystem, PowerSystemPlugin};

pub mod prelude {
    pub use crate::{
        components::{PowerBar, PowerLevel, PowerRegeneration},
        events::{
            ApplyLimitEvent, KnockedOutEvent, LevelUpEvent, LiftLimitEvent, PowerChangeEvent,
            ReviveEvent, SpendPowerEvent,
        },
        limits::{LimitType, PowerLimit, PowerLimits},
        plugin::{PowerBundle, PowerSystem, PowerSystemPlugin},
    };
}
