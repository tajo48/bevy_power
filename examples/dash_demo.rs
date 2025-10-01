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
        .add_systems(
            Update,
            (
                handle_input,
                update_player_movement,
                update_ui,
                update_dash_cooldown,
                handle_keyboard_toggle,
                update_method_status,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player {
    velocity: Vec2,
    dash_cooldown: Timer,
    last_dash_attempt: f32,
}

#[derive(Component)]
struct DashUI;

#[derive(Component)]
struct StatusMessage;

#[derive(Component)]
struct MethodStatusText;

#[derive(Resource)]
struct DashSettings {
    power_cost: f32,
    dash_force: f32,
    move_speed: f32,
}

impl Default for DashSettings {
    fn default() -> Self {
        Self {
            power_cost: 10.0,
            dash_force: 500.0,
            move_speed: 200.0,
        }
    }
}

fn setup(mut commands: Commands) {
    // Insert dash settings resource
    commands.insert_resource(DashSettings::default());
    // Insert limit method toggle resource
    commands.insert_resource(LimitMethodToggle::default());

    // Camera
    commands.spawn(Camera2d::default());

    // Spawn player with power system and movement
    commands.spawn((
        // Visual representation
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
        // Power system
        PowerBundle::with_max_power(100.0),
        // Player movement
        Player {
            velocity: Vec2::ZERO,
            dash_cooldown: Timer::from_seconds(1.0, TimerMode::Once),
            last_dash_attempt: 0.0,
        },
    ));

    // Create UI
    create_ui(&mut commands);
}

fn create_ui(commands: &mut Commands) {
    // Root container
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            // Title and instructions
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Dash Ability Demo"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent.spawn((
                        Text::new("Use WASD to move, SPACE to dash (costs 10 power)"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    parent.spawn((
                        Text::new("Dash has a 1-second cooldown and requires >10 power"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));

                    parent
                        .spawn((
                            Text::new("Method: try_limit (safe)"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.0, 1.0, 0.5)),
                        ))
                        .insert(MethodStatusText);
                });

            // Dash status display
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Text::new("Dash: Ready"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.0, 1.0, 0.0)),
                        ))
                        .insert(DashUI);

                    parent
                        .spawn((
                            Text::new(""),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.8, 0.0)),
                        ))
                        .insert(StatusMessage);
                });

            // Controls reminder
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Additional Controls:"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent.spawn((
                        Text::new("R - Regenerate 20 power  |  L - Apply power limit  |  C - Clear limits  |  T - Toggle method"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
        });
}

fn handle_input(
    mut player_query: Query<&mut Player, With<Player>>,
    mut power_system: PowerSystem,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<DashSettings>,
    time: Res<Time>,
    toggle: Res<LimitMethodToggle>,
) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    // Handle movement input
    let mut movement = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    if movement != Vec2::ZERO {
        movement = movement.normalize();
        player.velocity = movement * settings.move_speed;
    } else {
        // Apply friction when not moving
        player.velocity *= 0.9;
    }

    // Handle dash input
    if keyboard.just_pressed(KeyCode::Space) {
        player.last_dash_attempt = time.elapsed_secs();

        if player.dash_cooldown.is_finished() {
            // Try to spend power for dash
            if power_system.try_spend(settings.power_cost) {
                // Dash successful - add dash velocity
                let dash_direction = if movement != Vec2::ZERO {
                    movement
                } else {
                    Vec2::new(1.0, 0.0) // Default dash forward
                };

                player.velocity += dash_direction * settings.dash_force;
                player.dash_cooldown.reset();

                info!("Dash successful! Power spent: {}", settings.power_cost);
            } else {
                info!(
                    "Dash failed: Not enough power (need {})",
                    settings.power_cost
                );
            }
        } else {
            info!("Dash failed: Still on cooldown");
        }
    }

    // Additional controls
    if keyboard.just_pressed(KeyCode::KeyR) {
        power_system.change(20.0);
        info!("Regenerated 20 power");
    }

    if keyboard.just_pressed(KeyCode::KeyL) {
        if toggle.use_try_methods {
            if power_system.try_limit_points(
                1,
                30.0,
                Color::srgba(0.8, 0.0, 0.8, 0.7),
                Some(5.0), // 5 second duration
                false,
            ) {
                info!("Applied power limit (30 points for 5 seconds) using try_method");
            } else {
                info!("Failed to apply power limit using try_method");
            }
        } else {
            power_system.limit_points(
                1,
                30.0,
                Color::srgba(0.8, 0.0, 0.8, 0.7),
                Some(5.0), // 5 second duration
                false,
            );
            info!("Applied power limit (30 points for 5 seconds) using force method");
        }
    }

    if keyboard.just_pressed(KeyCode::KeyC) {
        power_system.lift(1);
        info!("Cleared power limits");
    }
}

fn update_player_movement(mut player_query: Query<(&mut Player, &mut Transform)>, time: Res<Time>) {
    let Ok((mut player, mut transform)) = player_query.single_mut() else {
        return;
    };

    let delta = time.delta_secs();

    // Update position
    transform.translation.x += player.velocity.x * delta;
    transform.translation.y += player.velocity.y * delta;

    // Keep player on screen (roughly)
    let bounds = Vec2::new(600.0, 400.0);
    transform.translation.x = transform.translation.x.clamp(-bounds.x, bounds.x);
    transform.translation.y = transform.translation.y.clamp(-bounds.y, bounds.y);

    // Apply velocity dampening
    player.velocity *= 0.95;
}

fn update_dash_cooldown(mut player_query: Query<&mut Player>, time: Res<Time>) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    player.dash_cooldown.tick(time.delta());
}

fn update_ui(
    player_query: Query<&Player, With<Player>>,
    power_query: Query<&PowerBar>,
    mut dash_ui_query: Query<&mut Text, (With<DashUI>, Without<StatusMessage>)>,
    mut status_query: Query<&mut Text, (With<StatusMessage>, Without<DashUI>)>,
    settings: Res<DashSettings>,
    time: Res<Time>,
    toggle: Res<LimitMethodToggle>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    let Ok(power_bar) = power_query.single() else {
        return;
    };

    let can_afford = !power_bar.is_knocked_out && power_bar.current > settings.power_cost;

    // Update dash status
    if let Ok(mut dash_text) = dash_ui_query.single_mut() {
        if power_bar.is_knocked_out {
            **dash_text = "Dash: KNOCKED OUT".to_string();
        } else if !player.dash_cooldown.is_finished() {
            let remaining =
                player.dash_cooldown.duration().as_secs_f32() - player.dash_cooldown.elapsed_secs();
            **dash_text = format!("Dash: Cooldown ({:.1}s)", remaining);
        } else if !can_afford {
            **dash_text = format!("Dash: Need {} Power", settings.power_cost);
        } else {
            **dash_text = "Dash: Ready".to_string();
        }
    }

    // Update status message
    if let Ok(mut status_text) = status_query.single_mut() {
        let time_since_attempt = time.elapsed_secs() - player.last_dash_attempt;

        if time_since_attempt < 2.0 {
            if player.dash_cooldown.elapsed_secs() < 0.1 {
                **status_text = "DASH ACTIVATED!".to_string();
            } else if !can_afford {
                **status_text = format!(
                    "Insufficient power! (Need {}, Have {:.0})",
                    settings.power_cost, power_bar.current
                );
            } else if !player.dash_cooldown.is_finished() {
                **status_text = "Dash on cooldown!".to_string();
            }
        } else {
            **status_text = format!(
                "Method: {}",
                if toggle.use_try_methods {
                    "try_limit (safe)"
                } else {
                    "limit (force)"
                }
            );
        }
    }
}

fn handle_keyboard_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut toggle: ResMut<LimitMethodToggle>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
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

fn update_method_status(
    toggle: Res<LimitMethodToggle>,
    mut method_text: Query<&mut Text, With<MethodStatusText>>,
) {
    if toggle.is_changed() {
        if let Ok(mut text) = method_text.single_mut() {
            **text = if toggle.use_try_methods {
                "Method: try_limit (safe)".to_string()
            } else {
                "Method: limit (force)".to_string()
            };
        }
    }
}
