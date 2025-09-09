# Bevy Power System

A flexible and feature-rich power management system for Bevy games. This crate provides components, events, and systems for managing player power/energy/mana with regeneration, limits, knockouts, leveling, and UI visualization.

## Features

- **Power Management**: Track current/max power with automatic bounds checking
- **Regeneration System**: Configurable power regeneration with delays and ramp-up
- **Power Limits**: Apply temporary or permanent power restrictions
- **Knockout System**: Handle zero-power states with revival mechanics
- **Level Progression**: Experience-based leveling with power bonuses
- **Visual UI**: Built-in power bar with limit visualization
- **Event-Driven**: Clean event-based API for all power operations
- **Safe Operations**: Try-methods that prevent invalid state changes

## Quick Start

### 1. Add to Cargo.toml

```toml
[dependencies]
bevy = "0.16.1"
bevy_power = "0.1.0"
```

### 2. Basic Setup

```rust
use bevy::prelude::*;
use bevy_power::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PowerSystemPlugin)  // Add the power system
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    // Spawn player with power system
    commands.spawn((
        PowerBundle::with_max_power(100.0),  // 100 max power
        Player,
    ));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut power_system: PowerSystem,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Try to spend 20 power (safe - won't cause knockout)
        if power_system.try_spend(20.0) {
            println!("Used special ability!");
        } else {
            println!("Not enough power!");
        }
    }
}
```

## Core Components

### PowerBar
Tracks current and maximum power values.

```rust
#[derive(Component)]
pub struct PowerBar {
    pub current: f32,      // Current power amount
    pub max: f32,          // Current maximum (affected by limits)
    pub base_max: f32,     // Base maximum (before limits)
    pub is_knocked_out: bool,
}

// Usage
let mut power_bar = PowerBar::new(100.0);
power_bar.spend(25.0);  // Returns true if successful
power_bar.add(10.0);    // Add power (clamped to max)
```

### PowerRegeneration
Handles automatic power recovery over time.

```rust
#[derive(Component)]
pub struct PowerRegeneration {
    pub regen_delay: f32,     // Delay before regen starts (default: 2.5s)
    pub base_rate: f32,       // Starting regen rate (default: 5.0/s)
    pub max_rate: f32,        // Maximum regen rate (default: 20.0/s)
    pub ramp_speed: f32,      // How fast regen accelerates
    // ... other fields
}
```

### PowerLimits
Manages temporary or permanent power restrictions.

```rust
// Apply a 20-point limit that expires in 5 seconds
power_system.limit_points(
    player_entity,
    1,                    // limit ID
    20.0,                 // points to reduce
    Color::RED,           // UI color
    Some(5.0),           // duration (None = permanent)
    false,               // reset regen cooldown?
    true,                // prevent regeneration?
);
```

### PowerLevel
Tracks experience and level progression.

```rust
#[derive(Component)]
pub struct PowerLevel {
    pub level: u32,
    pub experience: f32,
    pub experience_to_next: f32,
}
```

## Power System API

The `PowerSystem` provides a convenient interface for all power operations:

```rust
fn example_system(
    mut power_system: PowerSystem,
) {
    // Safe operations (return bool for success)
    if power_system.try_spend(30.0) {
        println!("Spell cast successfully!");
    }

    if power_system.can_afford(50.0) {
        println!("Can afford ultimate ability");
    }

    // Limit operations with safe variants
    if power_system.try_limit_points(1, 25.0, Color::PURPLE, Some(10.0), false, false) {
        println!("Curse applied!");
    } else {
        println!("Target immune to curse!");
    }

    // Direct operations (always execute)
    power_system.change(15.0);  // Add power
    power_system.revive(50.0);  // Revive from knockout
    power_system.lift(1);       // Remove limit by ID
}
```

## Examples

### Basic Usage
See `examples/simple_demo.rs` for keyboard controls and basic power operations.

### Interactive Demo
Run `examples/power_demo.rs` for a full GUI demonstration:
```bash
cargo run --example power_demo
```

### Game Integration
See `examples/dash_demo.rs` for a practical game ability system.

## Advanced Features

### Multiple Limit Types

```rust
// Fixed point reduction
power_system.limit_points(1, 20.0, Color::RED, None, false, false);

// Percentage-based reduction
power_system.limit_percentage(2, 25.0, Color::YELLOW, Some(5.0), true, true);
```

### Timed Limits with Auto-Expiry

```rust
// This limit will automatically remove itself after 10 seconds
power_system.limit_points(
    1,
    30.0,           // Reduce max power by 30
    Color::PURPLE,  // UI visualization color
    Some(10.0),     // Duration in seconds
    true,           // Reset regeneration cooldown when applied
    false,          // Don't prevent regeneration
);
```

### Custom Power Bundles

```rust
// Create custom power configurations
let custom_bundle = PowerBundle::custom(
    150.0,  // max power
    1.0,    // regen delay (seconds)
    8.0,    // base regen rate
    30.0,   // max regen rate
);

commands.spawn((custom_bundle, Player));
```

### Event Handling

```rust
fn handle_knockouts(
    mut knockout_events: EventReader<KnockedOutEvent>,
    mut level_up_events: EventReader<LevelUpEvent>,
) {
    for event in knockout_events.read() {
        println!("Entity {:?} was knocked out!", event.entity);
        // Play death sound, trigger respawn timer, etc.
    }

    for event in level_up_events.read() {
        println!("Level up! New level: {}, Power bonus: {}",
                 event.new_level, event.power_bonus);
    }
}
```

## UI Integration

The crate includes a built-in power bar UI that automatically:
- Shows current/max power values
- Visualizes active limits with colored segments
- Changes color based on power state (low, regenerating, knocked out)
- Updates in real-time

The UI is automatically added when you include the `PowerSystemPlugin`.

## Configuration

### Regeneration Settings
```rust
let mut regen = PowerRegeneration {
    regen_delay: 3.0,      // 3 second delay before regen starts
    base_rate: 10.0,       // Start regenerating at 10/second
    max_rate: 25.0,        // Cap at 25/second
    ramp_speed: 1.5,       // How quickly regen accelerates
    ..default()
};
```

### System Ordering

The plugin uses system sets to ensure proper ordering:
- `PowerSystemSet::Input` - Event handling
- `PowerSystemSet::Update` - Core logic updates
- `PowerSystemSet::UI` - Visual updates

## Events

All power operations generate events for maximum flexibility:

- `SpendPowerEvent` - Power spending attempts
- `PowerChangeEvent` - Direct power modifications
- `ApplyLimitEvent` - Limit applications
- `LiftLimitEvent` - Limit removals
- `KnockedOutEvent` - Knockout notifications
- `ReviveEvent` - Revival requests
- `LevelUpEvent` - Level progression

## Safe vs Unsafe Operations

The system provides two approaches for most operations:

**Safe Methods** (recommended for gameplay):
- `try_spend(amount)` - Won't cause knockout
- `try_limit_points(id, points, color, duration, resets_cooldown, stops_regen)` - Won't apply if it would cause knockout
- `can_afford(amount)` - Check before spending

**Direct Methods** (for system/admin use):
- `spend(amount)` - Always attempts to spend
- `limit_points(id, points, color, duration, resets_cooldown, stops_regen)` - Always applies limit
- `change(amount)` - Direct power modification

All methods now automatically operate on the entity with `PowerBar` component - no need to specify which entity!

## Performance

- Minimal overhead when no limits are active
- Efficient timer updates for temporary limits
- UI only updates when power state changes
- Event-driven architecture prevents unnecessary polling

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
