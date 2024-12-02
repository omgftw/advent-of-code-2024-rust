use std::fs;

#[cfg(test)]
mod tests;

pub(crate) async fn day2(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day2/data/main.txt").unwrap());


    let lines = data.lines().collect::<Vec<&str>>();

    let reports = lines.iter()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut mode;

    let mut results_part1: Vec<bool> = Vec::new();
    let mut results_part2: Vec<bool> = Vec::new();

    'line: for line in reports {
        if line[1] > line[0] {
            mode = "increasing";
        } else if line[1] < line[0] {
            mode = "decreasing";
        } else {
            results_part1.push(false);
            continue 'line;
        }
        let mut previous_level = line[0];
        for i in 1..line.len() {
            let level = line[i];
            if level > previous_level && mode == "decreasing" {
                results_part1.push(false);
                continue 'line;
            } else if level < previous_level && mode == "increasing" {
                results_part1.push(false);
                continue 'line;
            }
            let diff = (level - previous_level).abs();
            if diff < 1 || diff > 3 {
                results_part1.push(false);
                continue 'line;
            }
            previous_level = level;
        }
        results_part1.push(true);
    }

    let safe_reports_part1 = results_part1.iter().filter(|&result| *result).count() as i32;

    (safe_reports_part1, 0)
}
