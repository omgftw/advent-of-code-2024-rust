use crate::day6;
use std::fs;

#[tokio::test]
async fn test_day6_test_data() {
    println!("Running test...");
    let test_data = fs::read_to_string("src/day6/data/test_1.txt").unwrap();
    let (part1, part2) = day6::day6(Some(test_data)).await;

    // Part 1
    assert_eq!(part1, 41);
    // Part 2
    assert_eq!(part2, 6);
}

#[tokio::test]
async fn test_day6() {
    let (part1, part2) = day6::day6(None).await;

    // Part 1
    assert_eq!(part1, 5312);
    // Part 2
    assert_eq!(part2, 1748);
}
