use bevy::prelude::*;
use bevy_power::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PowerSystemPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_keyboard_input)
        .add_systems(Update, display_power_info)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PowerInfo;

fn setup(mut commands: Commands) {
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
                                    T - Apply timed limit (5 seconds)\n\
                                    1 - Remove limit 1\n\
                                    2 - Remove limit 2\n\
                                    3 - Remove limit 3\n\
                                    R - Remove all limits\n\
                                    V - Revive player\n\
                                    \n\
                                    Power regenerates after 2.5s of not spending\n\
                                    Timer limits will expire automatically",
                ))
                .insert(TextColor(Color::WHITE));
        });

    commands.insert_resource(PowerInfo);
}

fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    mut spend_events: EventWriter<SpendPowerEvent>,
    mut change_events: EventWriter<PowerChangeEvent>,
    mut limit_events: EventWriter<ApplyLimitEvent>,
    mut lift_events: EventWriter<LiftLimitEvent>,
    mut revive_events: EventWriter<ReviveEvent>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    // Space - Spend 10 power
    if keyboard.just_pressed(KeyCode::Space) {
        spend_events.write(SpendPowerEvent {
            entity: player_entity,
            amount: 10.0,
        });
        info!("Spent 10 power");
    }

    // S - Spend 30 power
    if keyboard.just_pressed(KeyCode::KeyS) {
        spend_events.write(SpendPowerEvent {
            entity: player_entity,
            amount: 30.0,
        });
        info!("Spent 30 power");
    }

    // A - Add 20 power
    if keyboard.just_pressed(KeyCode::KeyA) {
        change_events.write(PowerChangeEvent {
            entity: player_entity,
            amount: 20.0,
        });
        info!("Added 20 power");
    }

    // L - Apply fixed limit
    if keyboard.just_pressed(KeyCode::KeyL) {
        limit_events.write(ApplyLimitEvent::points(
            player_entity,
            1,
            20.0,
            Color::srgba(0.8, 0.0, 0.8, 0.7),
            None,
            false,
            false,
        ));
        info!("Applied 20 point limit");
    }

    // P - Apply percentage limit
    if keyboard.just_pressed(KeyCode::KeyP) {
        limit_events.write(ApplyLimitEvent::percentage(
            player_entity,
            2,
            25.0,
            Color::srgba(0.8, 0.8, 0.0, 0.7),
            None,
            true,
            true,
        ));
        info!("Applied 25% limit (resets cooldown & stops regen)");
    }

    // T - Apply timed limit
    if keyboard.just_pressed(KeyCode::KeyT) {
        limit_events.write(ApplyLimitEvent::points(
            player_entity,
            3,
            15.0,
            Color::srgba(0.0, 0.8, 0.8, 0.7),
            Some(5.0),
            false,
            false,
        ));
        info!("Applied timed limit (5 seconds) - will auto-expire");
    }

    // 1 - Remove limit 1
    if keyboard.just_pressed(KeyCode::Digit1) {
        lift_events.write(LiftLimitEvent {
            entity: player_entity,
            id: 1,
        });
        info!("Removed limit 1");
    }

    // 2 - Remove limit 2
    if keyboard.just_pressed(KeyCode::Digit2) {
        lift_events.write(LiftLimitEvent {
            entity: player_entity,
            id: 2,
        });
        info!("Removed limit 2");
    }

    // 3 - Remove limit 3
    if keyboard.just_pressed(KeyCode::Digit3) {
        lift_events.write(LiftLimitEvent {
            entity: player_entity,
            id: 3,
        });
        info!("Removed limit 3");
    }

    // R - Remove all limits
    if keyboard.just_pressed(KeyCode::KeyR) {
        for id in 1..=3 {
            lift_events.write(LiftLimitEvent {
                entity: player_entity,
                id,
            });
        }
        info!("Removed all limits");
    }

    // V - Revive
    if keyboard.just_pressed(KeyCode::KeyV) {
        revive_events.write(ReviveEvent {
            entity: player_entity,
            power_amount: 50.0,
        });
        info!("Revived with 50 power");
    }
}

fn display_power_info(
    power_query: Query<(&PowerBar, &PowerRegeneration, &PowerLimits), With<Player>>,
    time: Res<Time>,
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
                "Power: {:.1}/{:.1} (base: {:.1}) | KO: {} | Regen: {} | Limits: {} (timers: {})",
                power_bar.current,
                power_bar.max,
                power_bar.base_max,
                power_bar.is_knocked_out,
                regen.is_active,
                limits.limits.len(),
                active_timers
            );
        }
    }
}
