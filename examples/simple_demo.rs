use bevy::prelude::*;
use bevy_power::prelude::*;

#[derive(Resource, Default)]
struct LimitMethodToggle {
    use_try_methods: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PowerSystemPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_keyboard_input, display_power_info))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PowerInfo;

fn setup(mut commands: Commands) {
    // Insert the toggle resource
    commands.insert_resource(LimitMethodToggle::default());

    // Camera
    commands.spawn(Camera2d::default());

    // Spawn player entity with power components
    commands
        .spawn((
            PowerBar::new(100.0),
            PowerLevel::default(),
            PowerRegeneration::default(),
            PowerLimits::default(),
        ))
        .insert(Player);

    // Create simple UI
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Text::new(
                    "Power System Demo\n\
                                    \n\
                                    Controls:\n\
                                    SPACE - Spend 10 power\n\
                                    S - Spend 30 power\n\
                                    A - Add 20 power\n\
                                    L - Apply limit (20 points)\n\
                                    P - Apply percentage limit (25%)\n\
                                    T - Apply timed limit (5 seconds) / Toggle method\n\
                                    1 - Remove limit 1\n\
                                    2 - Remove limit 2\n\
                                    3 - Remove limit 3\n\
                                    R - Remove all limits\n\
                                    V - Revive player\n\
                                    \n\
                                    Power regenerates after 2.5s of not spending\n\
                                    Timer limits will expire automatically\n\
                                    T key: Toggle between try_limit (safe) and limit (force) methods"
                ))
                .insert(TextColor(Color::WHITE));
        });

    commands.insert_resource(PowerInfo);
}

fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    mut power_system: PowerSystem,
    mut toggle: ResMut<LimitMethodToggle>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    // Space - Try to spend 10 power
    if keyboard.just_pressed(KeyCode::Space) {
        if power_system.try_spend(player_entity, 10.0) {
            info!("Successfully spent 10 power");
        } else {
            info!("Failed to spend 10 power - insufficient power!");
        }
    }

    // S - Try to spend 30 power
    if keyboard.just_pressed(KeyCode::KeyS) {
        if power_system.try_spend(player_entity, 30.0) {
            info!("Successfully spent 30 power");
        } else {
            info!("Failed to spend 30 power - insufficient power!");
        }
    }

    // A - Add 20 power
    if keyboard.just_pressed(KeyCode::KeyA) {
        power_system.change(player_entity, 20.0);
        info!("Added 20 power");
    }

    // L - Apply fixed limit
    if keyboard.just_pressed(KeyCode::KeyL) {
        if toggle.use_try_methods {
            if power_system.try_limit_points(
                player_entity,
                1,
                20.0,
                Color::srgba(0.8, 0.0, 0.8, 0.7),
                None,
                false,
                false,
            ) {
                info!("Successfully applied 20 point limit (try_method)");
            } else {
                info!("Failed to apply point limit (try_method)");
            }
        } else {
            power_system.limit_points(
                player_entity,
                1,
                20.0,
                Color::srgba(0.8, 0.0, 0.8, 0.7),
                None,
                false,
                false,
            );
            info!("Applied 20 point limit (force method)");
        }
    }

    // P - Apply percentage limit
    if keyboard.just_pressed(KeyCode::KeyP) {
        if toggle.use_try_methods {
            if power_system.try_limit_percentage(
                player_entity,
                2,
                25.0,
                Color::srgba(0.8, 0.8, 0.0, 0.7),
                None,
                true,
                true,
            ) {
                info!("Successfully applied 25% limit (try_method, resets cooldown & stops regen)");
            } else {
                info!("Failed to apply percentage limit (try_method)");
            }
        } else {
            power_system.limit_percentage(
                player_entity,
                2,
                25.0,
                Color::srgba(0.8, 0.8, 0.0, 0.7),
                None,
                true,
                true,
            );
            info!("Applied 25% limit (force method, resets cooldown & stops regen)");
        }
    }

    // T - Toggle method or apply timed limit (depending on modifier)
    if keyboard.just_pressed(KeyCode::KeyT) {
        // Check if Shift is held to apply timed limit, otherwise toggle method
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            if toggle.use_try_methods {
                if power_system.try_limit_points(
                    player_entity,
                    3,
                    15.0,
                    Color::srgba(0.0, 0.8, 0.8, 0.7),
                    Some(5.0),
                    false,
                    false,
                ) {
                    info!("Successfully applied timed limit (try_method, 5 seconds) - will auto-expire");
                } else {
                    info!("Failed to apply timed limit (try_method)");
                }
            } else {
                power_system.limit_points(
                    player_entity,
                    3,
                    15.0,
                    Color::srgba(0.0, 0.8, 0.8, 0.7),
                    Some(5.0),
                    false,
                    false,
                );
                info!("Applied timed limit (force method, 5 seconds) - will auto-expire");
            }
        } else {
            // Toggle method
            toggle.use_try_methods = !toggle.use_try_methods;
            info!(
                "Switched to {} method",
                if toggle.use_try_methods {
                    "try_limit (safe)"
                } else {
                    "limit (force)"
                }
            );
        }
    }

    // 1 - Remove limit 1
    if keyboard.just_pressed(KeyCode::Digit1) {
        power_system.lift(player_entity, 1);
        info!("Removed limit 1");
    }

    // 2 - Remove limit 2
    if keyboard.just_pressed(KeyCode::Digit2) {
        power_system.lift(player_entity, 2);
        info!("Removed limit 2");
    }

    // 3 - Remove limit 3
    if keyboard.just_pressed(KeyCode::Digit3) {
        power_system.lift(player_entity, 3);
        info!("Removed limit 3");
    }

    // R - Remove all limits
    if keyboard.just_pressed(KeyCode::KeyR) {
        for id in 1..=3 {
            power_system.lift(player_entity, id);
        }
        info!("Removed all limits");
    }

    // V - Revive
    if keyboard.just_pressed(KeyCode::KeyV) {
        power_system.revive(player_entity, 50.0);
        info!("Revived with 50 power");
    }
}

fn display_power_info(
    power_query: Query<(&PowerBar, &PowerRegeneration, &PowerLimits), With<Player>>,
    time: Res<Time>,
    toggle: Res<LimitMethodToggle>,
) {
    static mut LAST_LOG: f32 = 0.0;

    let Ok((power_bar, regen, limits)) = power_query.single() else {
        return;
    };

    // Log power status every second
    unsafe {
        LAST_LOG += time.delta_secs();
        if LAST_LOG >= 1.0 {
            LAST_LOG = 0.0;

            let active_timers = limits
                .limits
                .iter()
                .filter(|l| l.duration.is_some())
                .count();

            info!(
                "Power: {:.1}/{:.1} (base: {:.1}) | KO: {} | Regen: {} | Limits: {} (timers: {}) | Method: {}",
                power_bar.current,
                power_bar.max,
                power_bar.base_max,
                power_bar.is_knocked_out,
                regen.is_active,
                limits.limits.len(),
                active_timers,
                if toggle.use_try_methods { "try_limit" } else { "limit" }
            );
        }
    }
}
