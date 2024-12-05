use std::fs;

#[cfg(test)]
mod tests;


// #[derive(Clone)]
// struct VectorChar {
//     char: char,
//     vector: (isize, isize),
// }

fn find_vectors(data: &[Vec<char>], start_pos: (usize, usize), char: char) -> Vec<(isize, isize)> {
    // check each position from start_pos from (-1, -1) to (1, 1)
    let start_pos = (start_pos.0 as isize, start_pos.1 as isize);
    let mut vectors = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            let pos = (start_pos.0 + x, start_pos.1 + y);
            if pos.0 < 0 || pos.1 < 0 {
                continue;
            }
            if pos.1 >= data.len() as isize || pos.0 >= data[0].len() as isize {
                continue;
            }
            if data[pos.1 as usize][pos.0 as usize] == char {
                vectors.push((x, y));
            }
        }
    }
    vectors
}

fn find_next_char(data: &[Vec<char>], start_pos: (usize, usize), vector: (isize, isize)) -> Option<(char, (usize, usize))> {
    let start_pos = (start_pos.0 as isize, start_pos.1 as isize);
    let new_pos = (start_pos.0 + vector.0, start_pos.1 + vector.1); 
    if new_pos.0 < 0 || new_pos.1 < 0 {
        return None;
    }
    if new_pos.1 >= data.len() as isize || new_pos.0 >= data[0].len() as isize {
        return None;
    }
    Some((data[new_pos.1 as usize][new_pos.0 as usize], (new_pos.0 as usize, new_pos.1 as usize)))
}

fn find_chars(data: &[Vec<char>], char: char) -> Vec<(usize, usize)> {
    let mut positions = vec![];
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == char {
                positions.push((x, y));
            }
        }
    }
    positions
}

// fn rotate_vector_map(vector_map: &[VectorChar]) -> Vec<VectorChar> {
//     vector_map.iter().map(|vector_char| VectorChar { char: vector_char.char, vector: (-vector_char.vector.1, vector_char.vector.0) }).collect()
// }

// fn check_vector_map(data: &[Vec<char>], start_pos: (usize, usize), vector_map: &Vec<VectorChar>) -> bool {
//     let start_pos = (start_pos.0 as isize, start_pos.1 as isize);
//     for vector_char in vector_map {
//         let new_pos = (start_pos.0 + vector_char.vector.0, start_pos.1 + vector_char.vector.1);
//         // Check bounds
//         if new_pos.0 < 0 || new_pos.1 < 0 {
//             return false;
//         }
//         if new_pos.1 >= data.len() as isize || new_pos.0 >= data[0].len() as isize {
//             return false;
//         }
//         if data[new_pos.1 as usize][new_pos.0 as usize] != vector_char.char {
//             return false;
//         }
//     }
//     true
// }

// fn check_rotational_vector_map(data: &[Vec<char>], start_pos: (usize, usize), vector_map: &[VectorChar]) -> bool {
//     for rotation in 0..4 {
//         let mut valid = true;
//         let start_pos = (start_pos.0 as isize, start_pos.1 as isize);
        
//         'check: for vector_char in vector_map {
//             // Apply rotation transformation inline
//             let rotated_vector = match rotation {
//                 0 => vector_char.vector,                                    // 0°   - (x, y)
//                 1 => (-vector_char.vector.1, vector_char.vector.0),        // 90°  - (-y, x)
//                 2 => (-vector_char.vector.0, -vector_char.vector.1),       // 180° - (-x, -y)
//                 3 => (vector_char.vector.1, -vector_char.vector.0),        // 270° - (y, -x)
//                 _ => unreachable!(),
//             };
            
//             let new_pos = (start_pos.0 + rotated_vector.0, start_pos.1 + rotated_vector.1);
            
//             // Check bounds and character match
//             if new_pos.0 < 0 || new_pos.1 < 0 
//                 || new_pos.1 >= data.len() as isize 
//                 || new_pos.0 >= data[0].len() as isize 
//                 || data[new_pos.1 as usize][new_pos.0 as usize] != vector_char.char {
//                 valid = false;
//                 break 'check;
//             }
//         }
        
//         if valid {
//             return true;
//         }
//     }
//     false
// }

// fn print_vector_map(vector_map: &Vec<VectorChar>) {
//     println!("{}.{}", vector_map.iter().find(|x| x.vector == (-1, -1)).unwrap().char, vector_map.iter().find(|x| x.vector == (1, -1)).unwrap().char);
//     println!(".{}.", vector_map.iter().find(|x| x.vector == (0, 0)).unwrap().char);
//     println!("{}.{}", vector_map.iter().find(|x| x.vector == (-1, 1)).unwrap().char, vector_map.iter().find(|x| x.vector == (1, 1)).unwrap().char);
// }

pub(crate) async fn day4(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day4/data/main.txt").unwrap());


    let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    
    let match_string = "XMAS".chars().collect::<Vec<char>>();
    
    let mut full_matches = 0;
    let start_positions = find_chars(&data, match_string[0]);
    for start_pos in start_positions {
        let vectors = find_vectors(&data, start_pos, match_string[1]);
        for vector in vectors {
            let mut next_pos = start_pos;
            let mut valid_vector = true;
            for match_char in match_string.iter().skip(1) {
                if let Some(next_char) = find_next_char(&data, next_pos, vector) {
                    if next_char.0 == *match_char {
                        next_pos = next_char.1;
                    } else {
                        valid_vector = false;
                        break;
                    }
                } else {
                    valid_vector = false;
                    break;
                }
            }
            if valid_vector {
                full_matches += 1;
            }
        }
    }

    // Part 2

    // A relative vector map of:
    // M.S
    // .A.
    // M.S
    // let vector_map = vec![
    //     VectorChar { char: 'M', vector: (-1, -1) },
    //     VectorChar { char: 'M', vector: (-1, 1) },
    //     VectorChar { char: 'A', vector: (0, 0) },
    //     VectorChar { char: 'S', vector: (1, 1) },
    //     VectorChar { char: 'S', vector: (1, -1) },
    // ];

    let mut matches = 0;
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            // This is inefficient.
            // if check_rotational_vector_map(&data, (x, y), &vector_map) {
            //     matches += 1;
            // }

            // This is about twice as fast, but with the volume of data we have, it's irrelevant.
            if data[y][x] == 'A' {
                if y > 0 && y < data.len() - 1 && x > 0 && x < data[y].len() - 1 {
                    // Count M's and S's in the diagonal positions
                    let mut m_count = 0;
                    let mut s_count = 0;
                    
                    // Check all four diagonal positions
                    for &pos in &[(y-1, x-1), (y+1, x+1), (y-1, x+1), (y+1, x-1)] {
                        match data[pos.0][pos.1] {
                            'M' => m_count += 1,
                            'S' => s_count += 1,
                            _ => (),
                        }
                    }
                    
                    if m_count == 2 && s_count == 2 {
                        // Check that opposite corners aren't identical
                        if data[y - 1][x - 1] != data[y + 1][x + 1] {
                            matches += 1;
                        }
                    }
                }
            }
        }
    }

    (full_matches, matches)
}
