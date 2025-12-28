use bevy::prelude::*;
use poke_engine::overworld::*;

mod demo;

use demo::map_loader::JsonMap;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Overworld,
    Battle,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct DemoOverworldSet;

#[derive(Component, Copy, Clone, Debug)]
pub struct Player;

#[derive(Resource)]
struct DemoMap(JsonMap);

#[derive(Resource)]
struct EncounterSettings {
    rate: f32,
}

#[derive(Component)]
struct BattleUi;

fn battle_enter(mut commands: Commands) {
    commands.spawn((
        BattleUi,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.12)),
    ));

    println!("Entered battle state. Press space to return")
}

fn battle_exit(mut commands: Commands, query: Query<Entity, With<BattleUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn battle_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Overworld);
    }
}

fn overworld_enter_input_lock(mut commands: Commands, input: Option<Res<OverworldInputSettings>>) {
    let Some(input) = input else {
        return;
    };

    commands.insert_resource(OverworldInputLock {
        timer: Timer::from_seconds(input.input_cooldown, TimerMode::Once),
    });
}

fn spawn_map(mut commands: Commands, demo: Res<DemoMap>, grid: Res<OverworldGridSettings>) {
    let map = &demo.0;

    for (y, row) in map.rows.iter().enumerate() {
        let wy = (map.height as usize - 1) - y;

        for (x, ch) in row.chars().enumerate() {
            let def = map
                .legend
                .get(&ch)
                .unwrap_or_else(|| panic!("Legend missing tile {ch}"));

            let [r, g, b] = def.color;
            let color = Color::srgb(r, g, b);

            commands.spawn((
                Sprite::from_color(color, Vec2::splat(grid.tile_size)),
                Transform::from_xyz(x as f32 * grid.tile_size, wy as f32 * grid.tile_size, 0.0),
            ));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OverworldPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, (setup, spawn_map).chain())
        .configure_sets(Update, OverworldSet.run_if(in_state(GameState::Overworld)))
        .configure_sets(
            Update,
            DemoOverworldSet
                .after(OverworldSet)
                .run_if(in_state(GameState::Overworld)),
        )
        .add_systems(
            Update,
            (
                handle_step_finished_for_encounter,
                demo_player_facing_color,
                camera_follow,
            )
                .chain()
                .in_set(DemoOverworldSet),
        )
        .add_systems(
            OnEnter(GameState::Battle),
            (snap_entity_to_grid, battle_enter).chain(),
        )
        .add_systems(Update, battle_input.run_if(in_state(GameState::Battle)))
        .add_systems(OnExit(GameState::Battle), battle_exit)
        .add_systems(
            OnEnter(GameState::Overworld),
            (snap_entity_to_grid, overworld_enter_input_lock).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, OverworldCamera));

    let json = demo::JsonMap::load("assets/tallgrass.json");
    json.validate();

    let overworld_map = build_overworld_map(&json);
    let tile_size = json.tile_size as f32;

    commands.insert_resource(overworld_map);

    commands.insert_resource(EncounterSettings { rate: 0.3 });
    commands.insert_resource(OverworldGridSettings { tile_size });
    commands.insert_resource(DemoMap(json));

    commands.insert_resource(OverworldMovementSettings {
        step_time: 0.16,
        turn_grace: 0.12,
    });
    commands.insert_resource(OverworldInputSettings {
        input_cooldown: 0.16,
    });
    commands.insert_resource(CameraFollowSettings { stiffness: 16.0 });

    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(tile_size)),
        Transform::from_xyz(2.0 * tile_size, 2.0 * tile_size, 10.0),
        Facing::South,
        Player,
        CameraTarget,
        OverworldPlayerController,
        GridPos { x: 2, y: 2 },
    ));
}

fn demo_player_facing_color(mut q: Query<(&Facing, &mut Sprite), (With<Player>, Changed<Facing>)>) {
    for (facing, mut sprite) in &mut q {
        sprite.color = match facing {
            Facing::North => Color::srgb(0.2, 0.6, 1.0),
            Facing::East => Color::srgb(1.0, 0.2, 0.2),
            Facing::South => Color::srgb(0.2, 0.4, 0.2),
            Facing::West => Color::srgb(1.0, 1.0, 0.2),
        };
    }
}

fn handle_step_finished_for_encounter(
    mut steps: MessageReader<StepFinished>,
    map: Res<OverworldMap>,
    settings: Res<EncounterSettings>,
    input: Res<OverworldInputSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    query: Query<&GridPos>,
) {
    for step in steps.read() {
        let Ok(grid) = query.get(step.entity) else {
            continue;
        };

        if map.is_encounter(grid.x, grid.y) {
            println!("Entered grass at {}, {}", grid.x, grid.y);

            if fastrand::f32() < settings.rate {
                commands.insert_resource(OverworldInputLock {
                    timer: Timer::from_seconds(input.input_cooldown, TimerMode::Once),
                });
                next_state.set(GameState::Battle);
                break;
            }
        }
    }
}

pub fn build_overworld_map(json: &demo::JsonMap) -> OverworldMap {
    let width = i32::try_from(json.width).expect("Width too large");
    let height = i32::try_from(json.height).expect("Height too large");

    let mut map = OverworldMap::new(width, height);

    for (y, row) in json.rows.iter().enumerate() {
        let wy = (json.height as usize - 1) - y;
        for (x, ch) in row.chars().enumerate() {
            let def = json
                .legend
                .get(&ch)
                .unwrap_or_else(|| panic!("Legend missing tile {ch}"));

            map.set_tile(x as i32, wy as i32, def.blocked, def.encounter);
        }
    }
    map
}
