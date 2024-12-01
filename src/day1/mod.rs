use std::fs;

#[cfg(test)]
mod tests;

fn part1(mut left_numbers: Vec<i32>, mut right_numbers: Vec<i32>) -> i32 {
    left_numbers.sort();
    right_numbers.sort();
    
    let mut total = 0;
    for i in 0..left_numbers.len() {
        total += (right_numbers[i] - left_numbers[i]).abs();
    }
    total
}

fn part2(left_numbers: Vec<i32>, right_numbers: Vec<i32>) -> i32 {
    let mut total = 0;
    for i in 0..left_numbers.len() {
        let mut occurences = 0;
        for j in 0..right_numbers.len() {
            if right_numbers[j] == left_numbers[i] {
                occurences += 1;
            }
        }
        total += left_numbers[i] * occurences;
    }
    total
}

pub(crate) async fn day1(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day1/data/main.txt").unwrap());
    
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in data.lines() {
        let mut nums = line.split_whitespace();
        left_numbers.push(nums.next().unwrap().parse::<i32>().unwrap());
        right_numbers.push(nums.next().unwrap().parse::<i32>().unwrap());
    }

    let total = part1(left_numbers.clone(), right_numbers.clone());
    let total2 = part2(left_numbers, right_numbers);
    (total, total2)
}
