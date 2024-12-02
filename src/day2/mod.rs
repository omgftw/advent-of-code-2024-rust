use std::fs;

#[cfg(test)]
mod tests;

fn is_report_valid(report: &Vec<i32>) -> bool {
    let mut mode = None;
    for window in report.windows(2) {
        let level = window[0];
        let next_level = window[1];
        if mode.is_none() {
            if next_level > level {
                mode = Some("increasing");
            } else if next_level < level {
                mode = Some("decreasing");
            } else {
                return false;
            }
        }
        if next_level > level && mode == Some("decreasing") {
            return false;
        } else if next_level < level && mode == Some("increasing") {
            return false;
        }
        let diff = (next_level - level).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}


fn is_report_valid_part2(report: &Vec<i32>) -> bool {
    for skip_idx in 0..report.len() {
        let filtered_report: Vec<_> = report[..skip_idx]
            .iter()
            .chain(report[skip_idx + 1..].iter())
            .copied()
            .collect();
        let is_valid = is_report_valid(&filtered_report);
        if is_valid {
            return true;
        }
    }
    false
}

pub(crate) async fn day2(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day2/data/main.txt").unwrap());


    let lines = data.lines().collect::<Vec<&str>>();

    let reports = lines.iter()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut results_part1: Vec<bool> = Vec::new();
    let mut results_part2: Vec<bool> = Vec::new();

    for report in reports {
        let valid = is_report_valid(&report);
        results_part1.push(valid);

        let valid = is_report_valid_part2(&report);
        results_part2.push(valid);
    }

    let safe_reports_part1 = results_part1.iter().filter(|&result| *result).count() as i32;
    let safe_reports_part2 = results_part2.iter().filter(|&result| *result).count() as i32;

    (safe_reports_part1, safe_reports_part2)
}
