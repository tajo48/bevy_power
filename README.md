# Bevy Power

A comprehensive power/energy management system for Bevy games, featuring regeneration, limits, knockouts, leveling, and visual UI components.

## Features

- **Power Management**: Track current/max power with automatic validation
- **Smart Regeneration**: Configurable delay and ramping regeneration rates
- **Power Limits**: Temporary or permanent power restrictions (points or percentage-based)
- **Knockout System**: Handle zero-power states with revival mechanics
- **Leveling System**: Experience-based progression with power bonuses
- **Visual UI**: Built-in power bar with limit visualization
- **Safe Operations**: Try-methods that prevent invalid states
- **Timer Support**: Auto-expiring limits and cooldown management
- **Event-Driven**: Fully event-based architecture for loose coupling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bevy_power = "0.1.0"
bevy = "0.16"
```

## Quick Start

```rust
use bevy::prelude::*;
use bevy_power::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PowerSystemPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    
    // Spawn entity with power system
    commands.spawn(PowerBundle::with_max_power(100.0));
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut power_system: PowerSystem,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Try to spend 10 power (safe method)
        if power_system.try_spend(10.0) {
            info!("Spent 10 power!");
        } else {
            info!("Not enough power!");
        }
    }
    
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Add 20 power
        power_system.change(20.0);
    }
}
```

## Core Concepts

### Power Bar

The `PowerBar` component tracks current and maximum power:

```rust
// Current power, max power, and base max (before limits)
pub struct PowerBar {
    pub current: f32,
    pub max: f32,        // Affected by limits
    pub base_max: f32,   // Original maximum
    pub is_knocked_out: bool,
}
```

### Power Regeneration

Configurable regeneration with delays and ramping:

```rust
let regen = PowerRegeneration {
    regen_delay: 2.5,      // Wait 2.5s after spending
    base_rate: 5.0,        // Start at 5 power/sec
    max_rate: 20.0,        // Ramp up to 20 power/sec
    ramp_speed: 2.0,       // How fast to ramp up
    ..default()
};
```

### Power Limits

Apply temporary or permanent restrictions:

```rust
// Safe method - only applies if it won't cause knockout
if power_system.try_limit_points(
    1,                                    // Unique ID
    20.0,                                // Reduce by 20 points
    Color::RED,                          // UI color
    Some(5.0),                          // Expires in 5 seconds
    false                               // Doesn't reset regen cooldown
) {
    info!("Limit applied successfully!");
}

// Percentage-based limits
power_system.try_limit_percentage(
    2,                                    // Different ID
    25.0,                                // Reduce by 25%
    Color::YELLOW,
    None,                               // Permanent until lifted
    true                                // Resets regeneration cooldown
);

// Remove limits
power_system.lift(1);  // Remove limit with ID 1
```

### Safe vs Force Methods

The system provides two approaches for applying limits:

```rust
// Safe methods - check if operation is valid first
if power_system.try_limit_points(1, 30.0, Color::RED, None, false) {
    // Only executes if it won't cause knockout
    info!("Limit safely applied");
}

// Force methods - always apply (may cause knockout)
power_system.limit_points(1, 30.0, Color::RED, None, false);
```

### Knockout and Revival

Handle zero-power states:

```rust
// Entities are knocked out when power reaches 0 or max power becomes 0
// Revival restores power and removes knockout state
power_system.revive(50.0);  // Revive with 50 power
```

## Power System API

The `PowerSystem` provides convenient access to all power operations:

```rust
fn game_system(mut power_system: PowerSystem) {
    // Spending power
    if power_system.can_afford(25.0) {
        power_system.try_spend(25.0);
    }
    
    // Changing power
    power_system.change(15.0);    // Add power
    power_system.change(-5.0);    // Subtract power
    
    // Applying limits
    power_system.try_limit_points(1, 20.0, Color::RED, Some(10.0), false);
    power_system.try_limit_percentage(2, 30.0, Color::BLUE, None, true);
    
    // Managing limits
    power_system.lift(1);         // Remove specific limit
    
    // Revival
    power_system.revive(75.0);
}
```

## Components and Bundles

### PowerBundle

Convenient bundle for spawning entities:

```rust
// Default settings (100 max power)
commands.spawn(PowerBundle::default());

// Custom max power
commands.spawn(PowerBundle::with_max_power(150.0));

// Full customization
commands.spawn(PowerBundle::custom(
    200.0,  // max_power
    3.0,    // regen_delay
    8.0,    // base_regen_rate  
    25.0,   // max_regen_rate
));
```

### Individual Components

You can also spawn components individually:

```rust
commands.spawn((
    PowerBar::new(100.0),
    PowerLevel::default(),
    PowerRegeneration::default(),
    PowerLimits::default(),
));
```

## Events

The system uses events for all operations:

```rust
// Listen to power events
fn handle_power_events(
    mut knockout_events: EventReader<KnockedOutEvent>,
    mut levelup_events: EventReader<LevelUpEvent>,
) {
    for event in knockout_events.read() {
        info!("Entity {:?} was knocked out!", event.entity);
    }
    
    for event in levelup_events.read() {
        info!("Level up! New level: {}, Power bonus: {}", 
              event.new_level, event.power_bonus);
    }
}
```

Available events:
- `SpendPowerEvent`
- `PowerChangeEvent` 
- `ApplyLimitEvent`
- `LiftLimitEvent`
- `KnockedOutEvent`
- `ReviveEvent`
- `LevelUpEvent`

## UI System

The plugin includes a visual power bar that automatically updates:

- Shows current/max power with text display
- Visualizes limits as colored segments
- Changes color based on power state (low, regenerating, knocked out)
- Displays base max in parentheses when limits are active

The UI is automatically created and positioned in the top-left corner. The power bar will appear for any entity with power components.

## Examples

### Running Examples

```bash
# Basic keyboard controls demo
cargo run --example simple_demo

# Full-featured UI demo with buttons
cargo run --example power_demo

# Practical dash ability implementation
cargo run --example dash_demo
```

### Dash Ability Example

Here's how you might implement a dash ability:

```rust
#[derive(Component)]
struct Player {
    dash_cooldown: Timer,
    velocity: Vec2,
}

fn handle_dash(
    mut player_query: Query<&mut Player>,
    mut power_system: PowerSystem,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player_query.single_mut() else { return };
    
    if keyboard.just_pressed(KeyCode::Space) && player.dash_cooldown.finished() {
        // Try to spend power for dash
        if power_system.try_spend(10.0) {
            player.velocity += Vec2::new(500.0, 0.0);  // Dash forward
            player.dash_cooldown.reset();
            info!("Dash successful!");
        } else {
            info!("Not enough power to dash!");
        }
    }
}
```

### Magic System Example

```rust
fn cast_spell(
    mut power_system: PowerSystem,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        // Fireball costs 15 power
        if power_system.try_spend(15.0) {
            spawn_fireball();
            info!("Fireball cast!");
        }
    }
    
    if keyboard.just_pressed(KeyCode::KeyH) {
        // Heal costs 20 power
        if power_system.try_spend(20.0) {
            heal_player();
            info!("Heal cast!");
        }
    }
}
```

### Debuff System Example

```rust
fn apply_poison_debuff(
    mut power_system: PowerSystem,
) {
    // Poison reduces max power by 30% for 10 seconds
    if power_system.try_limit_percentage(
        100,                    // Unique debuff ID
        30.0,                   // 30% reduction
        Color::srgb(0.5, 0.8, 0.2),  // Poison green
        Some(10.0),             // 10 seconds duration
        true                    // Resets regen (makes it worse)
    ) {
        info!("Player poisoned!");
    }
}
```

## Configuration

### Power Regeneration Settings

```rust
PowerRegeneration {
    regen_delay: 2.5,       // Seconds to wait after spending power
    base_rate: 5.0,         // Initial regeneration rate (power/sec)
    max_rate: 20.0,         // Maximum regeneration rate
    ramp_speed: 2.0,        // How quickly to ramp up (rate/sec)
    ..default()
}
```

### Power Level Settings

```rust
PowerLevel {
    level: 1,
    experience: 0.0,
    experience_to_next: 100.0,
}
```

Level-ups provide power bonuses with diminishing returns:
- Level 2: +20 power
- Level 3: +16 power  
- Level 4: +13.3 power
- etc.

## System Architecture

The power system uses Bevy's ECS architecture with:

- **Components**: Store power state (`PowerBar`, `PowerRegeneration`, etc.)
- **Systems**: Update power state and handle events
- **Events**: Communicate power changes between systems
- **Resources**: Global configuration and system access (`PowerSystem`)

This design allows for:
- Multiple entities with independent power systems
- Easy integration with other game systems
- Flexible event-based communication
- Clean separation of concerns

## Requirements

- Bevy 0.16+
- Rust 1.70+

## License

Licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.