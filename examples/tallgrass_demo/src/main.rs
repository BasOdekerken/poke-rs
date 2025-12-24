use bevy::prelude::*;
use poke_engine::overworld::*;

const TILE_SIZE: f32 = 16.0;
const STEP_TIME: f32 = 0.16;
const TURN_GRACE: f32 = 0.08;
const OVERWORLD_INPUT_COOLDOWN: f32 = 0.16;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
    Grass,
}

#[derive(Resource)]
struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(width: i32, height: i32, tiles: Vec<Tile>) -> Self {
        debug_assert!(width > 0 && height > 0);
        debug_assert!(tiles.len() == (width * height) as usize);
        Self {
            width,
            height,
            tiles,
        }
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn tile_at(&self, x: i32, y: i32) -> Option<Tile> {
        if !self.in_bounds(x, y) {
            return None;
        }
        Some(self.tiles[self.idx(x, y)])
    }

    fn is_blocked(&self, x: i32, y: i32) -> bool {
        matches!(self.tile_at(x, y), Some(Tile::Wall)) || !self.in_bounds(x, y)
    }

    fn is_grass(&self, x: i32, y: i32) -> bool {
        matches!(self.tile_at(x, y), Some(Tile::Grass))
    }
}

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

fn overworld_enter_input_lock(mut commands: Commands) {
    commands.insert_resource(OverworldInputLock {
        timer: Timer::from_seconds(OVERWORLD_INPUT_COOLDOWN, TimerMode::Once)
    });
}

fn tile_color(tile: Tile) -> Color {
    match tile {
        Tile::Floor => Color::srgb(0.7, 0.7, 0.7),
        Tile::Wall => Color::srgb(0.2, 0.2, 0.2),
        Tile::Grass => Color::srgb(0.0, 1.0, 0.0),
    }
}

fn spawn_map(mut commands: Commands, map: Res<Map>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.tile_at(x, y).unwrap();
            let color = tile_color(tile);

            commands.spawn((
                Sprite::from_color(color, Vec2::splat(TILE_SIZE)),
                Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
            ));
        }
    }
}

fn dir_pressed(keys: &ButtonInput<KeyCode>, dir: Facing) -> bool {
    match dir {
        Facing::North => keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW),
        Facing::East => keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD),
        Facing::South => keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS),
        Facing::West => keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA),
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OverworldPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, (setup, spawn_map).chain())
        .configure_sets(Update, OverworldSet.run_if(in_state(GameState::Overworld)))
        .add_systems(Update, (handle_step_finished_for_encounter, player_input_move, camera_follow).chain().in_set(DemoOverworldSet))
        .add_systems(OnEnter(GameState::Battle), (snap_entity_to_grid, battle_enter).chain())
        .add_systems(Update, battle_input.run_if(in_state(GameState::Battle)))
        .add_systems(OnExit(GameState::Battle), battle_exit)
        .add_systems(OnEnter(GameState::Overworld), (snap_entity_to_grid, overworld_enter_input_lock).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, OverworldCamera));

    commands.insert_resource(make_test_map());
    commands.insert_resource(EncounterSettings { rate: 0.3 });
    commands.insert_resource(OverworldGridSettings { tile_size: TILE_SIZE });
    commands.insert_resource(OverworldMovementSettings { step_time: STEP_TIME, turn_grace: TURN_GRACE });
    commands.insert_resource(OverworldInputSettings { input_cooldown: OVERWORLD_INPUT_COOLDOWN });
    commands.insert_resource(CameraFollowSettings { stiffness: 16.0 });

    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(TILE_SIZE)),
        Transform::from_xyz(2.0 * TILE_SIZE, 2.0 * TILE_SIZE, 10.0),
        Facing::South,
        Player,
        CameraTarget,
        GridPos { x: 2, y: 2 },
    ));
}

fn make_test_map() -> Map {
    let w = 10;
    let h = 8;

    let mut tiles = vec![Tile::Floor; (w * h) as usize];

    for x in 0..w {
        tiles[(0 * w + x) as usize] = Tile::Wall;
        tiles[((h - 1) * w + x) as usize] = Tile::Wall;
    }

    for y in 0..h {
        tiles[(y * w) as usize] = Tile::Wall;
        tiles[(y * w + (w - 1)) as usize] = Tile::Wall;
    }

    for y in 2..=4 {
        for x in 2..=5 {
            tiles[(y * w + x) as usize] = Tile::Grass;
        }
    }

    Map::new(w, h, tiles)
}

fn player_input_move(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<Map>,
    lock: Option<Res<OverworldInputLock>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut GridPos, &mut Facing, &mut Sprite, &Transform, Option<&mut TurnGrace>), (With<Player>, Without<MoveTween>)>,
) {
    let Ok((entity, mut pos, mut facing, mut sprite, transform, turn_grace)) = query.single_mut() else {
        return;
    };

    if lock.is_some() {
        return;
    }

    let desired_facing = if keys.pressed(KeyCode::ArrowUp) || (keys.pressed(KeyCode::KeyW)) {
        Some(Facing::North)
    } else if keys.pressed(KeyCode::ArrowDown) || (keys.pressed(KeyCode::KeyS)) {
        Some(Facing::South)
    } else if keys.pressed(KeyCode::ArrowLeft) || (keys.pressed(KeyCode::KeyA)) {
        Some(Facing::West)
    } else if keys.pressed(KeyCode::ArrowRight) || (keys.pressed(KeyCode::KeyD)) {
        Some(Facing::East)
    } else {
        None
    };

    let Some(dir) = desired_facing else {
        return;
    };

    if let Some(mut turn_grace) = turn_grace{
        if !dir_pressed(&keys, turn_grace.dir) {
            commands.entity(entity).remove::<TurnGrace>();
            return;
        }

        turn_grace.timer.tick(time.delta());

        if !turn_grace.timer.just_finished() {
            return;
        }

        commands.entity(entity).remove::<TurnGrace>();
    }

    if *facing != dir {
        *facing = dir;

        sprite.color = match dir {
            Facing::North => Color::srgb(0.2, 0.6, 1.0),
            Facing::East => Color::srgb(1.0, 0.2, 0.2),
            Facing::South => Color::srgb(0.2, 0.4, 0.2),
            Facing::West => Color::srgb(1.0, 1.0, 0.2),
        };

        commands.entity(entity).insert(TurnGrace {
            dir,
            timer: Timer::from_seconds(TURN_GRACE, TimerMode::Once),
        });

        return;
    }

    let (dx, dy) = match dir {
        Facing::North => (0, 1),
        Facing::East => (1, 0),
        Facing::South => (0, -1),
        Facing::West => (-1, 0),
    };

    let next_y = pos.y + dy;
    let next_x = pos.x + dx;

    if map.is_blocked(next_x, next_y) {
        return;
    }

    pos.x = next_x;
    pos.y = next_y;

    let from = Vec2::new(transform.translation.x, transform.translation.y);
    let to = Vec2::new(next_x as f32 * TILE_SIZE, next_y as f32 * TILE_SIZE);

    commands.entity(entity).insert(MoveTween {
        from,
        to,
        timer: Timer::from_seconds(STEP_TIME, TimerMode::Once),
    });
}

fn handle_step_finished_for_encounter(
    mut steps: MessageReader<StepFinished>,
    map: Res<Map>,
    settings: Res<EncounterSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    query: Query<&GridPos>,
) {
    for step in steps.read() {
        let Ok(grid) = query.get(step.entity) else {
            continue;
        };

        if map.is_grass(grid.x, grid.y) {
            println!("Entered grass at {}, {}", grid.x, grid.y);

            if fastrand::f32() < settings.rate {
                commands.insert_resource(OverworldInputLock {
                    timer: Timer::from_seconds(OVERWORLD_INPUT_COOLDOWN, TimerMode::Once)
                });
                next_state.set(GameState::Battle);
            }
        }
    }
}