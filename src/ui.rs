use crate::{
    components::{PowerBar, PowerRegeneration},
    limits::PowerLimits,
};
use bevy::prelude::*;

/// UI component for the power bar display
#[derive(Component)]
pub struct PowerBarUI;

/// Component marking the fill portion of the power bar
#[derive(Component)]
pub struct PowerBarFill;

/// Component marking the background of the power bar
#[derive(Component)]
pub struct PowerBarBackground;

/// Component marking limit segments in the power bar
#[derive(Component)]
pub struct PowerLimitSegment;

/// Component marking the power text display
#[derive(Component)]
pub struct PowerTextDisplay;

/// Setup the power bar UI
pub fn setup_power_ui(mut commands: Commands) {
    use bevy::ui::*;

    // Root UI container
    commands
        .spawn(Node {
            width: Val::Px(304.0),
            height: Val::Px(40.0),
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            position_type: PositionType::Absolute,
            padding: UiRect::all(Val::Px(2.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::srgb(0.1, 0.1, 0.1)))
        .insert(PowerBarUI)
        .with_children(|parent| {
            // Border/frame (pixelart style)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                })
                .insert(BackgroundColor(Color::NONE))
                .insert(BorderColor::all(Color::srgb(0.8, 0.8, 0.8)))
                .insert(PowerBarBackground) // Add this component to the frame for easy access
                .with_children(|parent| {
                    // Background
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        })
                        .insert(BackgroundColor(Color::srgb(0.2, 0.2, 0.2)));

                    // Power fill
                    parent
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            ..default()
                        })
                        .insert(BackgroundColor(Color::srgb(0.0, 0.8, 0.2)))
                        .insert(PowerBarFill);
                });

            // Text overlay (outside the frame so it's always visible)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .insert(BackgroundColor(Color::NONE))
                .with_children(|parent| {
                    parent
                        .spawn(Text::new("100 / 100"))
                        .insert(PowerTextDisplay)
                        .insert(TextFont {
                            font_size: 14.0,
                            ..default()
                        })
                        .insert(TextColor(Color::WHITE));
                });
        });
}

/// Update the power bar UI based on power state
pub fn update_power_bar_ui(
    power_query: Query<(&PowerBar, Option<&PowerLimits>, &PowerRegeneration)>,
    mut fill_query: Query<&mut Node, (With<PowerBarFill>, Without<PowerBarBackground>)>,
    mut bg_query: Query<&mut BackgroundColor, With<PowerBarFill>>,
    mut text_query: Query<&mut Text, With<PowerTextDisplay>>,
    mut commands: Commands,
    frame_query: Query<Entity, With<PowerBarBackground>>,
    existing_segments: Query<Entity, With<PowerLimitSegment>>,
) {
    // Get the first power bar entity (for single player)
    let Ok((power_bar, limits, regen)) = power_query.single() else {
        return;
    };

    // Update fill width - show current power relative to base_max
    if let Ok(mut node) = fill_query.single_mut() {
        let fill_percentage = if power_bar.base_max > 0.0 {
            (power_bar.current / power_bar.base_max * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        node.width = Val::Percent(fill_percentage);
    }

    // Update fill color based on state
    if let Ok(mut bg_color) = bg_query.single_mut() {
        bg_color.0 = if power_bar.is_knocked_out {
            Color::srgb(0.5, 0.0, 0.0) // Red when knocked out
        } else if regen.is_active {
            Color::srgb(0.0, 0.9, 0.4) // Bright green when regenerating
        } else if power_bar.percentage() < 0.3 {
            Color::srgb(0.8, 0.4, 0.0) // Orange when low
        } else {
            Color::srgb(0.0, 0.8, 0.2) // Normal green
        };
    }

    // Update text - show current/max but also indicate base_max if different
    if let Ok(mut text) = text_query.single_mut() {
        **text = if power_bar.is_knocked_out {
            "KNOCKED OUT".to_string()
        } else if power_bar.max < power_bar.base_max {
            format!(
                "{:.0} / {:.0} ({:.0})",
                power_bar.current, power_bar.max, power_bar.base_max
            )
        } else {
            format!("{:.0} / {:.0}", power_bar.current, power_bar.max)
        };
    }

    // Clean up existing limit segments
    for entity in existing_segments.iter() {
        commands.entity(entity).despawn();
    }

    // Handle limit segments
    if let Some(limits) = limits {
        if frame_query.single().is_ok() {
            // Create limit segments that show missing power from the right side
            let segments = limits.get_limit_segments(power_bar.base_max);
            let mut offset_from_right = 0.0;
            let bar_width = 300.0; // Total bar width minus padding

            for (color, percentage) in segments.iter() {
                let segment_width = (percentage * bar_width).min(bar_width - offset_from_right);

                if segment_width > 0.0 {
                    // Add segments to the power bar frame
                    if let Ok(frame_entity) = frame_query.single() {
                        commands.entity(frame_entity).with_children(|parent| {
                            parent
                                .spawn(Node {
                                    width: Val::Px(segment_width),
                                    height: Val::Percent(100.0),
                                    right: Val::Px(offset_from_right),
                                    top: Val::Px(0.0),
                                    position_type: PositionType::Absolute,
                                    ..default()
                                })
                                .insert(BackgroundColor(color.with_alpha(0.7)))
                                .insert(PowerLimitSegment);
                        });
                    }

                    offset_from_right += segment_width;

                    // Don't go beyond the bar width
                    if offset_from_right >= bar_width {
                        break;
                    }
                }
            }
        }
    }
}

// Helper function for creating pixelart borders can be added here if needed
// Currently not used in the implementation
