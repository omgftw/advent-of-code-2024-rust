use std::{collections::HashSet, fs};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl std::ops::Add for &Vector2 {
    type Output = Vector2;

    fn add(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for &Vector2 {
    type Output = Vector2;

    fn sub(self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub(crate) async fn day8(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day8/data/main.txt").unwrap());

    let map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut antennas = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != '.' {
                antennas.push((
                    map[y][x],
                    Vector2 {
                        x: x as i32,
                        y: y as i32,
                    },
                ));
            }
        }
    }

    let mut pairs = Vec::new();
    for antenna in antennas.clone() {
        for antenna2 in antennas.clone() {
            if antenna.1 == antenna2.1 {
                continue;
            }
            if antenna.0 == antenna2.0 {
                pairs.push((antenna.clone(), antenna2.clone()));
            }
        }
    }

    let map_bounds = Vector2 {
        x: map[0].len() as i32,
        y: map.len() as i32,
    };

    let mut antinodes_part1 = HashSet::new();
    for (antenna_a, antenna_b) in pairs.clone() {
        let delta = &antenna_b.1 - &antenna_a.1;

        let antinode_a = &antenna_a.1 - &delta;
        if antinode_a.x >= 0 && antinode_a.x < map_bounds.x && antinode_a.y >= 0 && antinode_a.y < map_bounds.y {
            antinodes_part1.insert(antinode_a);
        }

        let antinode_b = &antenna_b.1 + &delta;
        if antinode_b.x >= 0 && antinode_b.x < map_bounds.x && antinode_b.y >= 0 && antinode_b.y < map_bounds.y {
            antinodes_part1.insert(antinode_b);
        }
    }

    let mut antinodes_part2 = HashSet::new();
    for (antenna_a, antenna_b) in pairs {
        antinodes_part2.insert(antenna_a.clone().1);
        antinodes_part2.insert(antenna_b.clone().1);

        let delta = &antenna_b.1 - &antenna_a.1;

        let mut antinode_a = &antenna_a.1 - &delta;
        loop {
            if antinode_a.x >= 0
                && antinode_a.x < map_bounds.x
                && antinode_a.y >= 0
                && antinode_a.y < map_bounds.y
            {
                antinodes_part2.insert(antinode_a.clone());
            } else {
                break;
            }
            antinode_a = &antinode_a - &delta;
        }

        let mut antinode_b = &antenna_b.1 + &delta;
        loop {
            if antinode_b.x >= 0
                && antinode_b.x < map_bounds.x
                && antinode_b.y >= 0
                && antinode_b.y < map_bounds.y
            {
                antinodes_part2.insert(antinode_b.clone());
            } else {
                break;
            }
            antinode_b = &antinode_b + &delta;
        }
    }

    // Print the map with antinodes marked
    // for y in 0..map.len() {
    //     for x in 0..map[y].len() {
    //         let pos = Vector2 { x: x as i32, y: y as i32 };
    //         if antinodes_part2.contains(&pos) {
    //             print!("#");
    //         } else {
    //             print!("{}", map[y][x]);
    //         }
    //     }
    //     println!();
    // }

    (antinodes_part1.len() as i32, antinodes_part2.len() as i32)
}
