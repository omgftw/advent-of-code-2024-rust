use std::fs;

use regex::Regex;

#[cfg(test)]
mod tests;

pub(crate) async fn day3(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day3/data/main.txt").unwrap());

    let mul_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let do_regex = Regex::new(r"(do\(\)|don't\(\))").unwrap();

    let do_matches = do_regex.find_iter(&data);
    let mut instructions = Vec::new();
    for d in do_matches {
        instructions.push((d.start(), if d.as_str() == "do()" { true } else { false }));
    }
    instructions.sort_by_key(|k| k.0);

    // let mut do_mult = true;
    let captures = mul_regex.captures_iter(&data);
    let mut total_part1 = 0;
    let mut total_part2 = 0;
    for c in captures {
        let a = c[1].parse::<i32>().unwrap();
        let b = c[2].parse::<i32>().unwrap();
        total_part1 += a * b;

        let capture_pos = c.get(0).unwrap().start();
        if instructions.iter().rev().find(|&&(pos, _)| pos < capture_pos).unwrap_or(&(0, true)).1 {
            total_part2 += a * b;
        }
    }

    (total_part1, total_part2)
}
