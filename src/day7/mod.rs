use std::{collections::HashMap, fs, sync::Mutex};
use lazy_static::lazy_static;
#[cfg(test)]
mod tests;

lazy_static!{
    static ref RESULTS: Mutex<HashMap<i64, String>> = Mutex::new(HashMap::new());
}

fn create_branch(new_value: i64, mut remaining_nums: Vec<&str>, signature: String, concat_operator: bool) -> i64 {
    if remaining_nums.len() == 0 {
        RESULTS.lock().unwrap().insert(new_value, signature);
        return new_value;
    }

    let first_num = remaining_nums.remove(0);
    let first_num_int = first_num.parse::<i64>().unwrap();
    create_branch(new_value + first_num_int, remaining_nums.clone(), signature.clone(), concat_operator);
    create_branch(new_value * first_num_int, remaining_nums.clone(), signature.clone(), concat_operator);
    if concat_operator {
        let concat_value = format!("{}{}", new_value, first_num);
        create_branch(concat_value.parse::<i64>().unwrap(), remaining_nums.clone(), signature.clone(), concat_operator);
    }

    0
}

pub(crate) async fn day7(data: Option<String>) -> (i64, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day7/data/main.txt").unwrap());
    
    let data = data.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let mut part1_sum = 0;

    for mut item in data.clone() {
        let signature = item.clone().join(" ");
        let expected_result = item.remove(0).replace(":", "").parse::<i64>().unwrap();
        let first_num = item.remove(0);
        let first_num_int = first_num.parse::<i64>().unwrap();
        let remaining_nums = item.clone();
        create_branch(first_num_int, remaining_nums, signature.clone(), false);
        {
            let results = RESULTS.lock().unwrap();
            if results.contains_key(&expected_result) {
                if results.get(&expected_result).unwrap() == &signature {
                    part1_sum += expected_result;
                }
            }
        }
    }

    {
        let mut results = RESULTS.lock().unwrap();
        results.clear();
    }
    let mut part2_sum = 0;
    for mut item in data {
        let signature = item.clone().join(" ");
        let expected_result = item.remove(0).replace(":", "").parse::<i64>().unwrap();
        let first_num = item.remove(0);
        let first_num_int = first_num.parse::<i64>().unwrap();
        let remaining_nums = item.clone();
        create_branch(first_num_int, remaining_nums, signature.clone(), true);
        {
            let results = RESULTS.lock().unwrap();
            if results.contains_key(&expected_result) {
                if results.get(&expected_result).unwrap() == &signature {
                    part2_sum += expected_result;
                }
            }
        }
    }

    (part1_sum, part2_sum)
}
