use std::{collections::HashSet, fs};
use rayon::prelude::*;

#[cfg(test)]
mod tests;

fn create_branch(
    new_value: i64,
    remaining_nums: &[&str],
    concat_operator: bool,
    results: &mut HashSet<i64>,
) {
    if remaining_nums.is_empty() {
        results.insert(new_value);
        return;
    }

    let first_num_int = remaining_nums[0].parse::<i64>().unwrap();
    let remaining = &remaining_nums[1..];

    // Add results from addition
    create_branch(
        new_value + first_num_int,
        remaining,
        concat_operator,
        results,
    );
    // Add results from multiplication
    create_branch(
        new_value * first_num_int,
        remaining,
        concat_operator,
        results,
    );

    if concat_operator {
        if let Ok(concat_value) = format!("{}{}", new_value, remaining_nums[0]).parse::<i64>() {
            create_branch(concat_value, remaining, concat_operator, results);
        }
    }
}

pub(crate) async fn day7(data: Option<String>) -> (i64, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day7/data/main.txt").unwrap());
    let data: Vec<Vec<&str>> = data
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    // Use par_iter() and map to process lines in parallel
    let (part1_sum, part2_sum): (i64, i64) = data.par_iter()
        .map(|line| {
            let expected_result = line[0].trim_end_matches(':').parse::<i64>().unwrap();
            let nums = &line[1..];
            let first_num = nums[0].parse::<i64>().unwrap();
            let mut results = HashSet::new();
            
            // Part 1
            create_branch(first_num, &nums[1..], false, &mut results);
            let part1 = if results.contains(&expected_result) {
                expected_result
            } else {
                0
            };
            
            // Part 2
            results.clear();
            create_branch(first_num, &nums[1..], true, &mut results);
            let part2 = if results.contains(&expected_result) {
                expected_result
            } else {
                0
            };
            
            (part1, part2)
        })
        .reduce(
            || (0, 0),
            |a, b| (a.0 + b.0, a.1 + b.1)
        );

    (part1_sum, part2_sum)
}
