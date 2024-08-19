use bevy::prelude::*;
use rand::prelude::*;

// Game state
#[derive(Resource)]
struct GameState {
    grid: Vec<Vec<Cell>>,
    size: usize,
    row_constraints: Vec<usize>,
    col_constraints: Vec<usize>,
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Tree,
    Tent,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(generate_random_level(6))
        .add_systems(Startup, setup) 
        .add_systems(Update, input_handling)
        .add_systems(Update, validate_grid)
        .run();
}

fn setup(mut commands: Commands, game_state: Res<GameState>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn grid cells
    for (y, row) in game_state.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = match cell {
                Cell::Empty => Color::Srgba(Srgba::WHITE),
                Cell::Tree => Color::Srgba(Srgba::GREEN),
                Cell::Tent => Color::Srgba(Srgba::RED),
            };

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    (x as f32 - game_state.size as f32 / 2.0) * 60.0,
                    (game_state.size as f32 / 2.0 - y as f32) * 60.0,
                    0.0,
                ),
                ..default()
            });
        }
    }

    // Spawn row constraints
    for (y, &constraint) in game_state.row_constraints.iter().enumerate() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                constraint.to_string(),
                TextStyle {
                    font_size: 30.0,
                    color: Color::Srgba(Srgba::BLACK),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(
                -(game_state.size as f32 / 2.0 + 0.5) * 60.0,
                (game_state.size as f32 / 2.0 - y as f32) * 60.0,
                0.0,
            ),
            ..default()
        });
    }

    // Spawn column constraints
    for (x, &constraint) in game_state.col_constraints.iter().enumerate() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                constraint.to_string(),
                TextStyle {
                    font_size: 30.0,
                    color: Color::Srgba(Srgba::BLACK),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(
                (x as f32 - game_state.size as f32 / 2.0) * 60.0,
                (game_state.size as f32 / 2.0 + 0.5) * 60.0,
                0.0,
            ),
            ..default()
        });
    }
}

fn input_handling(
    mut game_state: ResMut<GameState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        // let window = windows.get_primary().unwrap();
        let window = windows.single();
        
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let x = ((world_position.x + game_state.size as f32 * 30.0) / 60.0) as usize;
            let y = ((game_state.size as f32 * 30.0 - world_position.y) / 60.0) as usize;

            if x < game_state.size && y < game_state.size {
                if game_state.grid[y][x] == Cell::Empty {
                    game_state.grid[y][x] = Cell::Tent;
                } else if game_state.grid[y][x] == Cell::Tent {
                    game_state.grid[y][x] = Cell::Empty;
                }
            }
        }
    }
}

fn validate_grid(game_state: Res<GameState>) {
    let is_valid = is_grid_valid(&game_state);
    if is_valid {
        println!("The current grid state is valid!");
    } else {
        println!("The current grid state is not valid.");
    }
}

fn is_grid_valid(game_state: &GameState) -> bool {
    let size = game_state.size;

    // Check row and column constraints
    for i in 0..size {
        let row_tent_count = game_state.grid[i].iter().filter(|&&cell| cell == Cell::Tent).count();
        let col_tent_count = game_state.grid.iter().map(|row| row[i]).filter(|&cell| cell == Cell::Tent).count();

        if row_tent_count != game_state.row_constraints[i] || col_tent_count != game_state.col_constraints[i] {
            return false;
        }
    }

    // Check tent-tree adjacency and tent-tent non-adjacency
    for y in 0..size {
        for x in 0..size {
            if game_state.grid[y][x] == Cell::Tent {
                if !has_adjacent_tree(&game_state.grid, x, y) || has_adjacent_tent(&game_state.grid, x, y) {
                    return false;
                }
            }
        }
    }

    true
}

fn has_adjacent_tree(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let size = grid.len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions.iter() {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        if new_x >= 0 && new_x < size as i32 && new_y >= 0 && new_y < size as i32 {
            if grid[new_y as usize][new_x as usize] == Cell::Tree {
                return true;
            }
        }
    }
    false
}

fn has_adjacent_tent(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let size = grid.len();
    let directions = [
        (0, 1), (1, 1), (1, 0), (1, -1),
        (0, -1), (-1, -1), (-1, 0), (-1, 1)
    ];

    for (dx, dy) in directions.iter() {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        if new_x >= 0 && new_x < size as i32 && new_y >= 0 && new_y < size as i32 {
            if grid[new_y as usize][new_x as usize] == Cell::Tent {
                return true;
            }
        }
    }
    false
}

fn generate_random_level(size: usize) -> GameState {
    let mut rng = rand::thread_rng();
    let mut grid = vec![vec![Cell::Empty; size]; size];
    
    // Calculate the number of trees to place (about 20-30% of the grid)
    let tree_count = rng.gen_range((size * size / 5)..(size * size / 3));
    
    // Place trees randomly
    for _ in 0..tree_count {
        loop {
            let x = rng.gen_range(0..size);
            let y = rng.gen_range(0..size);
            if grid[y][x] == Cell::Empty {
                grid[y][x] = Cell::Tree;
                break;
            }
        }
    }
    
    // Ensure each tree has at least one adjacent empty cell for a tent
    for y in 0..size {
        for x in 0..size {
            if grid[y][x] == Cell::Tree {
                if !has_adjacent_empty(&grid, x, y) {
                    // If no adjacent empty cell, create one
                    create_adjacent_empty(&mut grid, x, y);
                }
            }
        }
    }

    // Place tents (this is a simple placement and might not result in a unique solution)
    for y in 0..size {
        for x in 0..size {
            if grid[y][x] == Cell::Tree && has_adjacent_empty(&grid, x, y) {
                place_adjacent_tent(&mut grid, x, y);
            }
        }
    }

    // Calculate constraints
    let row_constraints: Vec<usize> = grid.iter()
        .map(|row| row.iter().filter(|&&cell| cell == Cell::Tent).count())
        .collect();

    let col_constraints: Vec<usize> = (0..size)
        .map(|x| grid.iter().filter(|row| row[x] == Cell::Tent).count())
        .collect();

    // Remove tents for the player to solve
    for y in 0..size {
        for x in 0..size {
            if grid[y][x] == Cell::Tent {
                grid[y][x] = Cell::Empty;
            }
        }
    }

    GameState {
        grid,
        size,
        row_constraints,
        col_constraints,
    }
}

fn has_adjacent_empty(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    let size = grid.len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dx, dy) in directions.iter() {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        if new_x >= 0 && new_x < size as i32 && new_y >= 0 && new_y < size as i32 {
            if grid[new_y as usize][new_x as usize] == Cell::Empty {
                return true;
            }
        }
    }
    false
}

fn create_adjacent_empty(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let size = grid.len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut rng = rand::thread_rng();
    
    let available_directions: Vec<(i32, i32)> = directions
        .iter()
        .filter(|&&(dx, dy)| {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            new_x >= 0 && new_x < size as i32 && new_y >= 0 && new_y < size as i32
        })
        .cloned()
        .collect();
    
    if let Some(&(dx, dy)) = available_directions.choose(&mut rng) {
        let new_x = (x as i32 + dx) as usize;
        let new_y = (y as i32 + dy) as usize;
        grid[new_y][new_x] = Cell::Empty;
    }
}

fn place_adjacent_tent(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let size = grid.len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut rng = rand::thread_rng();
    
    let available_directions: Vec<(i32, i32)> = directions
        .iter()
        .filter(|&&(dx, dy)| {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            new_x >= 0 && new_x < size as i32 && new_y >= 0 && new_y < size as i32 && grid[new_y as usize][new_x as usize] == Cell::Empty
        })
        .cloned()
        .collect();
    
    if let Some(&(dx, dy)) = available_directions.choose(&mut rng) {
        let new_x = (x as i32 + dx) as usize;
        let new_y = (y as i32 + dy) as usize;
        grid[new_y][new_x] = Cell::Tent;
    }
}