use bevy::prelude::*;
use bevy_power::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PowerSystemPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_button_clicks, update_button_states))
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
}

#[derive(Component)]
struct ButtonLabel;

fn setup(mut commands: Commands) {
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
                Text::new("Power regenerates after 2.5s of not spending\nRegeneration ramps up over time\nTimed limits will expire automatically!"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
                        ))
                        .insert(DemoButton::ApplyPercentLimit)
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Limit 25%"),
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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
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
                            BorderColor(Color::WHITE),
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
    player_query: Query<Entity, With<Player>>,
    mut power_system: PowerSystem,
    mut power_level_query: Query<&mut PowerLevel, With<Player>>,
    mut status_text: Query<&mut Text, With<StatusText>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for (interaction, button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Visual feedback
                bg_color.0 = bg_color.0.with_luminance(0.3);

                // Update status text
                if let Ok(mut text) = status_text.single_mut() {
                    **text = match button {
                        DemoButton::SpendSmall => {
                            power_system.spend(player_entity, 10.0);
                            "Status: Spent 10 power".to_string()
                        }
                        DemoButton::SpendLarge => {
                            power_system.spend(player_entity, 30.0);
                            "Status: Spent 30 power".to_string()
                        }
                        DemoButton::AddPower => {
                            power_system.change(player_entity, 20.0);
                            "Status: Added 20 power".to_string()
                        }
                        DemoButton::ApplyPointsLimit => {
                            power_system.limit_points(
                                player_entity,
                                1,
                                20.0,
                                Color::srgba(0.8, 0.0, 0.8, 0.7),
                                None,
                                false,
                                false,
                            );
                            "Status: Applied 20 point limit".to_string()
                        }
                        DemoButton::ApplyPercentLimit => {
                            power_system.limit_percentage(
                                player_entity,
                                2,
                                25.0,
                                Color::srgba(0.8, 0.8, 0.0, 0.7),
                                None,
                                true, // This one resets cooldown
                                true, // This one stops regeneration
                            );
                            "Status: Applied 25% limit (resets cooldown & stops regen)".to_string()
                        }
                        DemoButton::ApplyTimedLimit => {
                            power_system.limit_points(
                                player_entity,
                                3,
                                15.0,
                                Color::srgba(0.0, 0.8, 0.8, 0.7),
                                Some(5.0), // 5 second duration
                                false,
                                false,
                            );
                            "Status: Applied timed limit (5s) - will auto-expire!".to_string()
                        }
                        DemoButton::LiftLimit { id } => {
                            power_system.lift(player_entity, *id);
                            format!("Status: Lifted limit {}", id)
                        }
                        DemoButton::Revive => {
                            power_system.revive(player_entity, 50.0);
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
        };
        bg_color.0 = base_color;
    }
}
