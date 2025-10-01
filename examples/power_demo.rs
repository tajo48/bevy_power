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
                handle_button_clicks,
                update_button_states,
                handle_keyboard_toggle,
                update_method_status,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
enum DemoButton {
    SpendSmall,
    SpendLarge,
    AddPower,
    ApplyPointsLimit,
    ApplyPercentLimit,
    ApplyTimedLimit,
    LiftLimit { id: u32 },
    Revive,
    LevelUp,
    ToggleLimitMethod,
}

#[derive(Component)]
struct ButtonLabel;

#[derive(Component)]
struct MethodStatusText;

fn setup(mut commands: Commands) {
    // Insert the toggle resource
    commands.insert_resource(LimitMethodToggle::default());

    // Camera
    commands.spawn(Camera2d::default());

    // Spawn player entity with power components
    commands
        .spawn(PowerBundle::custom(100.0, 2.5, 5.0, 20.0))
        .insert(Player);

    // Create demo UI with buttons
    create_demo_ui(&mut commands);
}

fn create_demo_ui(commands: &mut Commands) {
    // Root UI container for buttons
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            row_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Power System Demo"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Instructions
            parent.spawn((
                Text::new("Power regenerates after 2.5s of not spending\nRegeneration ramps up over time\nTimed limits will expire automatically!\n'Reset' limits pause regen for 2.5s\nT key: Toggle between try_limit (safe) and limit (always applies)"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // Method status
            parent
                .spawn((
                    Text::new("Method: try_limit (safe)"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 1.0, 0.5)),
                ))
                .insert(MethodStatusText);



            // Button rows
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Spend 10
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.8, 0.4, 0.0)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::SpendSmall)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Spend 10"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });

                    // Spend 30
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.8, 0.2, 0.0)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::SpendLarge)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Spend 30"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });

                    // Add 20
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.8, 0.2)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::AddPower)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Add 20"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Limit 20pts
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.8, 0.0, 0.8)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::ApplyPointsLimit)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Limit 20pts"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });

                    // Limit 25%
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.8, 0.8, 0.0)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::ApplyPercentLimit)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Limit 25% (reset)"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });

                    // Timed Limit (5s)
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.8, 0.8)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::ApplyTimedLimit)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Timed Limit (5s)"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Lift Limit 1
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.4, 0.4, 0.8)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::LiftLimit { id: 1 })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Lift Limit 1"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });

                    // Lift Limit 2
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.4, 0.4, 0.8)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::LiftLimit { id: 2 })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Lift Limit 2"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });

                    // Lift Limit 3
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.4, 0.4, 0.8)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::LiftLimit { id: 3 })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Lift Limit 3"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Revive
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::Revive)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Revive"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });

                    // Level Up
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.8, 0.6, 0.0)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::LevelUp)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Level Up"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });

                    // Toggle Method
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.5, 0.2, 0.8)),
                            BorderColor::all(Color::WHITE),
                        ))
                        .insert(DemoButton::ToggleLimitMethod)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Toggle Method"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ))
                                .insert(ButtonLabel);
                        });
                });

            // Status text
            parent
                .spawn(Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Text::new("Status: Ready"),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ))
                        .insert(StatusText);
                });
        });
}

#[derive(Component)]
struct StatusText;

fn handle_button_clicks(
    mut interaction_query: Query<
        (&Interaction, &DemoButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut power_system: PowerSystem,
    mut power_level_query: Query<&mut PowerLevel, With<Player>>,
    mut status_text: Query<&mut Text, With<StatusText>>,
    toggle: Res<LimitMethodToggle>,
) {
    for (interaction, button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Visual feedback
                bg_color.0 = bg_color.0.with_luminance(0.3);

                // Update status text
                if let Ok(mut text) = status_text.single_mut() {
                    **text = match button {
                        DemoButton::SpendSmall => {
                            if power_system.try_spend(10.0) {
                                "Status: Successfully spent 10 power".to_string()
                            } else {
                                "Status: Failed to spend 10 power - insufficient power!".to_string()
                            }
                        }
                        DemoButton::SpendLarge => {
                            if power_system.try_spend(30.0) {
                                "Status: Successfully spent 30 power".to_string()
                            } else {
                                "Status: Failed to spend 30 power - insufficient power!".to_string()
                            }
                        }
                        DemoButton::AddPower => {
                            power_system.change(20.0);
                            "Status: Added 20 power".to_string()
                        }
                        DemoButton::ApplyPointsLimit => {
                            if toggle.use_try_methods {
                                if power_system.try_limit_points(
                                    1,
                                    20.0,
                                    Color::srgba(0.8, 0.0, 0.8, 0.7),
                                    None,
                                    false,
                                ) {
                                    "Status: Successfully applied 20 point limit (try_method)"
                                        .to_string()
                                } else {
                                    "Status: Failed to apply point limit (try_method)".to_string()
                                }
                            } else {
                                power_system.limit_points(
                                    1,
                                    20.0,
                                    Color::srgba(0.8, 0.0, 0.8, 0.7),
                                    None,
                                    false,
                                );
                                "Status: Applied 20 point limit (force method)".to_string()
                            }
                        }
                        DemoButton::ApplyPercentLimit => {
                            if toggle.use_try_methods {
                                if power_system.try_limit_percentage(
                                    2,
                                    25.0,
                                    Color::srgba(0.8, 0.8, 0.0, 0.7),
                                    None,
                                    true, // This one resets cooldown
                                ) {
                                    "Status: Successfully applied 25% limit (try_method, resets cooldown)".to_string()
                                } else {
                                    "Status: Failed to apply percentage limit (try_method)"
                                        .to_string()
                                }
                            } else {
                                power_system.limit_percentage(
                                    2,
                                    25.0,
                                    Color::srgba(0.8, 0.8, 0.0, 0.7),
                                    None,
                                    true, // This one resets cooldown
                                );
                                "Status: Applied 25% limit (force method, resets cooldown)"
                                    .to_string()
                            }
                        }
                        DemoButton::ApplyTimedLimit => {
                            if toggle.use_try_methods {
                                if power_system.try_limit_points(
                                    3,
                                    15.0,
                                    Color::srgba(0.0, 0.8, 0.8, 0.7),
                                    Some(5.0), // 5 second duration
                                    false,
                                ) {
                                    "Status: Successfully applied timed limit (try_method, 5s) - will auto-expire!".to_string()
                                } else {
                                    "Status: Failed to apply timed limit (try_method)".to_string()
                                }
                            } else {
                                power_system.limit_points(
                                    3,
                                    15.0,
                                    Color::srgba(0.0, 0.8, 0.8, 0.7),
                                    Some(5.0), // 5 second duration
                                    false,
                                );
                                "Status: Applied timed limit (force method, 5s) - will auto-expire!"
                                    .to_string()
                            }
                        }
                        DemoButton::LiftLimit { id } => {
                            power_system.lift(*id);
                            format!("Status: Lifted limit {}", id)
                        }
                        DemoButton::Revive => {
                            power_system.revive(50.0);
                            "Status: Revived with 50 power".to_string()
                        }
                        DemoButton::LevelUp => {
                            // Manually trigger level up for demo
                            if let Ok(mut level) = power_level_query.single_mut() {
                                level.experience = level.experience_to_next;
                                format!("Status: Level up to {} triggered!", level.level + 1)
                            } else {
                                "Status: Failed to level up".to_string()
                            }
                        }
                        DemoButton::ToggleLimitMethod => {
                            format!(
                                "Status: Currently using {} method (press T or click to toggle)",
                                if toggle.use_try_methods {
                                    "try_limit (safe)"
                                } else {
                                    "limit (force)"
                                }
                            )
                        }
                    };
                }
            }
            Interaction::Hovered => {
                bg_color.0 = bg_color.0.with_luminance(0.5);
            }
            Interaction::None => {
                // Color is reset in update_button_states
            }
        }
    }
}

fn update_button_states(
    mut buttons: Query<(&DemoButton, &mut BackgroundColor), Without<Interaction>>,
) {
    // Reset button colors when not interacting
    for (button, mut bg_color) in &mut buttons {
        let base_color = match button {
            DemoButton::SpendSmall => Color::srgb(0.8, 0.4, 0.0),
            DemoButton::SpendLarge => Color::srgb(0.8, 0.2, 0.0),
            DemoButton::AddPower => Color::srgb(0.0, 0.8, 0.2),
            DemoButton::ApplyPointsLimit => Color::srgb(0.8, 0.0, 0.8),
            DemoButton::ApplyPercentLimit => Color::srgb(0.8, 0.8, 0.0),
            DemoButton::ApplyTimedLimit => Color::srgb(0.0, 0.8, 0.8),
            DemoButton::LiftLimit { .. } => Color::srgb(0.4, 0.4, 0.8),
            DemoButton::Revive => Color::srgb(0.2, 0.8, 0.2),
            DemoButton::LevelUp => Color::srgb(0.8, 0.6, 0.0),
            DemoButton::ToggleLimitMethod => Color::srgb(0.5, 0.2, 0.8),
        };
        bg_color.0 = base_color;
    }
}

fn handle_keyboard_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut toggle: ResMut<LimitMethodToggle>,
    mut interaction_query: Query<(&DemoButton, &mut BackgroundColor), With<Button>>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        toggle.use_try_methods = !toggle.use_try_methods;

        // Also trigger the toggle button visual feedback
        for (button, mut bg_color) in &mut interaction_query {
            if matches!(button, DemoButton::ToggleLimitMethod) {
                bg_color.0 = bg_color.0.with_luminance(0.3);
            }
        }

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
