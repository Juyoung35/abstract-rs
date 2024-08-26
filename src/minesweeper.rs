use bevy::prelude::*;
use rand::Rng;

const MINE_CHANCE: f32 = 0.15;
const GRID_SIZE: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellType {
    Mine,
    Clue(usize),
}

#[derive(Component)]
struct Cell {
    is_revealed: bool,
    cell_type: CellType,
}

#[derive(Resource)]
struct GameBoard {
    is_generated: bool,
    cells: Vec<Vec<Entity>>,
}

#[derive(Resource)]
struct GameState {
    is_game_over: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameBoard { cells: Vec::new(), is_generated: false })
        .insert_resource(GameState { is_game_over: false })
        .add_system(Startup, setup)
        .add_system(Update, cell_click)
        .run();
}

fn setup(mut commands: Commands, mut game_board: ResMut<GameBoard>) {
    commands.spawn(Camera2dBundle::default());

    for y in 0..GRID_SIZE {
        let mut row = Vec::new();
        for x in 0..GRID_SIZE {
            let cell = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::GRAY,
                            custom_size: Some(Vec2::new(30.0, 30.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 - GRID_SIZE as f32 / 2.0) * 32.0,
                            (y as f32 - GRID_SIZE as f32 / 2.0) * 32.0,
                            0.0,
                        ),
                        ..default()
                    },
                    Cell {
                        is_mine: false,
                        is_revealed: false,
                        adjacent_mines: 0,
                    },
                ))
                .id();
            row.push(cell);
        }
        game_board.cells.push(row);
    }
}


fn generate_puzzle(commands: &mut Commands, game_board: &mut GameBoard, start_x: usize, start_y: usize) {
    let mut rng = rand::thread_rng();

    // Place mines
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if x == start_x && y == start_y {
                continue; // Ensure the starting cell is not a mine
            }
            let is_mine = rng.gen::<f32>() < MINE_CHANCE;
            if let Ok(mut cell) = commands.get_entity(game_board.cells[y][x]) {
                cell.get_mut::<Cell>().unwrap().is_mine = is_mine;
            }
        }
    }

    // Calculate adjacent mines
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let mut count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < GRID_SIZE as i32 && ny >= 0 && ny < GRID_SIZE as i32 {
                        if let Ok(neighbor) = commands.get_entity(game_board.cells[ny as usize][nx as usize]) {
                            if neighbor.get::<Cell>().unwrap().is_mine {
                                count += 1;
                            }
                        }
                    }
                }
            }
            if let Ok(mut cell) = commands.get_entity(game_board.cells[y][x]) {
                cell.get_mut::<Cell>().unwrap().adjacent_mines = count;
            }
        }
    }

    game_board.is_generated = true;
}

fn reveal_cell(commands: &mut Commands, game_board: &GameBoard, x: usize, y: usize) -> bool {
    if x >= GRID_SIZE || y >= GRID_SIZE {
        return false;
    }

    let entity = game_board.cells[y][x];
    if let Ok(mut cell_entry) = commands.get_entity(entity) {
        let mut cell = cell_entry.get_mut::<Cell>().unwrap();
        if cell.is_revealed {
            return false;
        }

        cell.is_revealed = true;
        let mut sprite = cell_entry.get_mut::<Sprite>().unwrap();
        sprite.color = Color::WHITE;

        if cell.is_mine {
            sprite.color = Color::RED;
            return true; // Game over
        } else if cell.adjacent_mines > 0 {
            commands.entity(entity).with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        cell.adjacent_mines.to_string(),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                });
            });
        } else {
            // Reveal adjacent cells
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < GRID_SIZE as i32 && ny >= 0 && ny < GRID_SIZE as i32 {
                        reveal_cell(commands, game_board, nx as usize, ny as usize);
                    }
                }
            }
        }
    }
    false
}

fn cell_click(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    window: Res<Window>,
    mut game_board: ResMut<GameBoard>,
    mut game_state: ResMut<GameState>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    cell_query: Query<(Entity, &Transform, &Cell)>,
) {
    if game_state.is_game_over {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = windows.get_primary().unwrap();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            for (entity, transform, cell) in cell_query.iter() {
                let cell_pos = transform.translation.truncate();
                if world_position.distance(cell_pos) < 15.0 && !cell.is_revealed {
                    let grid_x = ((cell_pos.x + (GRID_SIZE as f32 / 2.0) * 32.0) / 32.0) as usize;
                    let grid_y = ((cell_pos.y + (GRID_SIZE as f32 / 2.0) * 32.0) / 32.0) as usize;

                    if !game_board.is_generated {
                        generate_puzzle(&mut commands, &mut game_board, grid_x, grid_y);
                    }

                    let is_game_over = reveal_cell(&mut commands, &game_board, grid_x, grid_y);
                    if is_game_over {
                        game_state.is_game_over = true;
                        println!("Game Over!");
                    }

                    break;
                }
            }
        }
    }
}