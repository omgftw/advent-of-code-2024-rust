use std::{collections::HashMap, fs};

#[cfg(test)]
mod tests;

fn sort_update<'a>(rules: &'a std::collections::HashMap<&str, Vec<&str>>, update: &'a Vec<&str>) -> Vec<&'a str> {
    let mut sorted: Vec<&str> = update.to_vec();
    sorted.sort_by(|a, b| {
        if let Some(a_rules) = rules.get(a) {
            if a_rules.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            std::cmp::Ordering::Equal
        }
    });
    sorted
}

pub(crate) async fn day5(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day5/data/main.txt").unwrap());
    let data = data.split("\n\n").collect::<Vec<&str>>();

    let rules_base: Vec<(&str, &str)> = data[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in rules_base {
        rules.entry(rule.0).or_insert(Vec::new()).push(rule.1);
    }

    let updates: Vec<Vec<&str>> = data[1]
        .lines()
        .map(|line| line.split(',').collect())
        .collect();

    let mut correct: Vec<&Vec<&str>> = Vec::new();
    let mut incorrect: Vec<&Vec<&str>> = Vec::new();

    let sorted_updates = updates.iter().map(|update| sort_update(&rules, &update)).collect::<Vec<Vec<&str>>>();

    for (i, sorted) in sorted_updates.iter().enumerate() {
        if sorted == &updates[i] {
            correct.push(&updates[i]);
        } else {
            incorrect.push(&sorted);
        }
    }

    let mut total = 0;
    for update in correct {
        let middle_index = update.len() / 2;
        total += update[middle_index].parse::<i32>().unwrap();
    }

    let mut total2 = 0;
    for update in incorrect {
        let sorted = sort_update(&rules, &update);
        let middle_index = sorted.len() / 2;
        total2 += sorted[middle_index].parse::<i32>().unwrap();
    }

    (total, total2)
}
