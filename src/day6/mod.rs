use std::collections::HashSet;
use std::fs;
use std::ops::Add;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Vector2d {
    x: isize,
    y: isize,
}

impl Add<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Vector2d) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Vector2d> for &Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: &Vector2d) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vector2d {
    fn x(&self) -> usize {
        self.x.try_into().unwrap_or(0)
    }

    fn y(&self) -> usize {
        self.y.try_into().unwrap_or(0)
    }
}

fn find_start_position(map: &Vec<Vec<char>>) -> Vector2d {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if ['>', 'v', '<', '^'].contains(&cell) {
                return Vector2d {
                    x: x as isize,
                    y: y as isize,
                };
            }
        }
    }
    Vector2d { x: 0, y: 0 }
}

fn get_movement_vector(direction: char) -> Vector2d {
    match direction {
        '>' => Vector2d { x: 1, y: 0 },
        'v' => Vector2d { x: 0, y: 1 },
        '<' => Vector2d { x: -1, y: 0 },
        '^' => Vector2d { x: 0, y: -1 },
        _ => Vector2d { x: 0, y: 0 },
    }
}

fn turn_90_degrees(direction: Vector2d) -> Vector2d {
    match direction {
        Vector2d { x: 1, y: 0 } => Vector2d { x: 0, y: 1 }, // > -> v
        Vector2d { x: 0, y: 1 } => Vector2d { x: -1, y: 0 }, // v -> <
        Vector2d { x: 0, y: -1 } => Vector2d { x: 1, y: 0 }, // < -> ^
        Vector2d { x: -1, y: 0 } => Vector2d { x: 0, y: -1 }, // ^ -> >
        _ => direction,
    }
}

fn get_next_tile(
    map: &Vec<Vec<char>>,
    position: &Vector2d,
    vector: &Vector2d,
) -> Result<Vector2d, ()> {
    let new_position = position + vector;
    if new_position.y >= map.len() as isize
        || new_position.y < 0
        || new_position.x >= map[0].len() as isize
        || new_position.x < 0
    {
        return Err(());
    }
    Ok(new_position)
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

pub(crate) async fn day6(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day6/data/main.txt").unwrap());

    let mut map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start_position = find_start_position(&map);
    let mut position = start_position;
    let initial_vector = get_movement_vector(map[position.y()][position.x()]);
    let mut vector = initial_vector;
    map[position.y()][position.x()] = 'X';

    while let Ok(next_tile) = get_next_tile(&map, &position, &vector) {
        if map[next_tile.y()][next_tile.x()] == '#' {
            vector = turn_90_degrees(vector);
        } else {
            position = next_tile;
            map[position.y()][position.x()] = 'X';
        }
    }

    let mut x_count = 0;
    for row in map.iter() {
        for &tile in row.iter() {
            if tile == 'X' {
                x_count += 1;
            }
        }
    }

    let mut loop_count = 0;
    let mut obstacles_encountered = HashSet::with_capacity(map.len() * map[0].len());
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let orig_symbol = map[y][x];
            map[y][x] = 'O';
            let mut position = start_position;
            let mut vector = initial_vector;
            obstacles_encountered.clear();
            
            while let Ok(next_tile) = get_next_tile(&map, &position, &vector) {
                let next_tile_symbol = map[next_tile.y()][next_tile.x()];
                if next_tile_symbol == '#' || next_tile_symbol == 'O' {
                    let obstacle = (next_tile, vector);
                    if obstacles_encountered.contains(&obstacle) {
                        loop_count += 1;
                        break;
                    }
                    obstacles_encountered.insert(obstacle);
                    vector = turn_90_degrees(vector);
                } else {
                    position = next_tile;
                }
            }
            map[y][x] = orig_symbol;
        }
    }

    (x_count, loop_count)
}
