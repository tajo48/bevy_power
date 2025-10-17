# Bevy Power System

A comprehensive power/energy system for Bevy games with regeneration, limits, knockouts, leveling, and UI components.

## Features

- **Power Management**: Track current and maximum power with automatic regeneration
- **Power Limits**: Apply temporary or permanent power restrictions with visual feedback
- **Knockout System**: Handle zero-power states with revival mechanics
- **Leveling System**: Experience-based progression with power bonuses
- **Regeneration**: Smart power recovery with delays and ramping
- **Built-in UI**: Animated power bar with limit visualization
- **Event-Driven**: Clean API using Bevy's message system

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_power = "0.1.1"
bevy = "0.17.2"
```

Basic setup:

```rust
use bevy::prelude::*;
use bevy_power::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PowerSystemPlugin)
        .add_systems(Startup, spawn_player)
        .run();
}

fn spawn_player(mut commands: Commands) {
    // Spawn entity with power system
    commands.spawn(PowerBundle::with_max_power(100.0));
}
```

## Core Components

### PowerBar
Tracks current and maximum power:

```rust
// Default: 100 max power
commands.spawn(PowerBundle::new());

// Custom max power
commands.spawn(PowerBundle::with_max_power(150.0));

// Full customization
commands.spawn(PowerBundle::custom(
    100.0, // max_power
    2.5,   // regen_delay
    5.0,   // base_regen_rate
    20.0,  // max_regen_rate
));
```

### Power Limits
Apply restrictions that reduce available power:

```rust
fn apply_debuff(mut power_system: PowerSystem) {
    // Fixed point reduction
    power_system.limit_points(
        1,                                    // limit_id
        20.0,                                // points to reduce
        Color::srgba(0.8, 0.0, 0.8, 0.7),   // UI color
        Some(5.0),                           // duration (5 seconds)
        false,                               // resets_cooldown
    );

    // Percentage-based reduction
    power_system.limit_percentage(
        2,                                    // limit_id
        25.0,                                // 25% reduction
        Color::srgba(0.8, 0.8, 0.0, 0.7),   // UI color
        None,                                // permanent
        true,                                // resets regeneration cooldown
    );
}
```

### Safe vs Force Methods
The crate provides both safe (try_*) and force methods:

```rust
// Safe methods - won't cause knockout
if power_system.try_spend(30.0) {
    println!("Power spent successfully");
} else {
    println!("Not enough power!");
}

if power_system.try_limit_points(1, 50.0, Color::RED, None, false) {
    println!("Limit applied safely");
} else {
    println!("Limit would cause knockout - not applied");
}

// Force methods - always execute
power_system.spend(30.0);  // May cause knockout
power_system.limit_points(1, 50.0, Color::RED, None, false);  // May cause knockout
```

## Power System API

The `PowerSystem` SystemParam provides convenient access to all power operations:

```rust
fn use_abilities(mut power_system: PowerSystem, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        // Try to cast spell
        if power_system.try_spend(25.0) {
            println!("Spell cast!");
        }
    }

    if input.just_pressed(KeyCode::KeyR) {
        // Add power pickup
        power_system.change(20.0);
    }

    if input.just_pressed(KeyCode::KeyL) {
        // Apply curse (reduces max power by 30 points for 10 seconds)
        power_system.limit_points(1, 30.0, Color::PURPLE, Some(10.0), true);
    }

    if input.just_pressed(KeyCode::KeyC) {
        // Remove curse
        power_system.lift(1);
    }
}
```

## Regeneration System

Power automatically regenerates after not spending for a configurable delay:

- **Delay**: Time before regeneration starts (default: 2.5 seconds)
- **Ramping**: Regeneration rate increases over time
- **Reset on Spend**: Using power resets the regeneration timer
- **Cooldown Reset**: Some limits can force regeneration to restart

```rust
// Customize regeneration in PowerBundle::custom()
PowerBundle::custom(
    100.0,  // max_power
    1.0,    // regen_delay (1 second)
    10.0,   // base_regen_rate (10 power/second)
    30.0,   // max_regen_rate (30 power/second max)
)
```

## Knockout System

When power reaches zero or max power becomes zero (due to limits):

```rust
fn handle_knockout(
    knocked_out: MessageReader<KnockedOutEvent>,
    mut power_system: PowerSystem,
) {
    for event in knocked_out.read() {
        println!("Player {} was knocked out!", event.entity);

        // Revive after 3 seconds with 50% power
        power_system.revive(50.0);
    }
}
```

## Examples

Run the included examples to see the system in action:

```bash
# Interactive demo with buttons
cargo run --example power_demo

# Dash ability implementation
cargo run --example dash_demo

# Simple keyboard controls
cargo run --example simple_demo
```

### Dash Demo Controls
- **WASD**: Move player
- **SPACE**: Dash (costs 10 power, 1-second cooldown)
- **R**: Regenerate 20 power
- **L**: Apply power limit
- **C**: Clear limits
- **T**: Toggle between safe/force methods

## UI System

The crate includes a built-in power bar UI that automatically:

- Shows current/max power with text
- Displays power percentage as a fill bar
- Changes color based on power state (low, regenerating, knocked out)
- Visualizes active limits as colored segments
- Updates in real-time

The UI is automatically added when you include the `PowerSystemPlugin`.

## Events

The system uses Bevy's message system for clean event handling:

```rust
fn listen_to_power_events(
    mut knocked_out: MessageReader<KnockedOutEvent>,
    mut level_up: MessageReader<LevelUpEvent>,
) {
    for event in knocked_out.read() {
        println!("Entity {} was knocked out!", event.entity);
    }

    for event in level_up.read() {
        println!("Leveled up to {}! Gained {} power!",
                 event.new_level, event.power_bonus);
    }
}
```

Available events:
- `KnockedOutEvent`
- `LevelUpEvent`
- `SpendPowerEvent`
- `PowerChangeEvent`
- `ApplyLimitEvent`
- `LiftLimitEvent`
- `ReviveEvent`

## System Architecture

The plugin is organized into system sets that run in order:

1. **PowerSystemSet::Input** - Handle events and input
2. **PowerSystemSet::Update** - Update power states, regeneration, limits
3. **PowerSystemSet::UI** - Update visual components

This ensures consistent execution order and allows you to schedule your systems appropriately.

## Use Cases

This system is perfect for:

- **RPGs**: Mana/energy systems with spell costs and regeneration
- **Action Games**: Stamina systems for abilities like dashing, jumping
- **Strategy Games**: Resource management with temporary modifiers
- **Survival Games**: Hunger/thirst systems with food effects
- **Fighting Games**: Special move meters with cooldowns
