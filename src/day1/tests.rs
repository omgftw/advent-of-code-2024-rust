use crate::day1;
use std::fs;

#[tokio::test]
async fn test_day1_test_data() {
    let test_data = fs::read_to_string("src/day1/data/test_1.txt").unwrap();
    let (part1, part2) = day1::day1(Some(test_data)).await;

    // Part 1
    assert_eq!(part1, 11);
    // Part 2
    assert_eq!(part2, 31);
}

#[tokio::test]
async fn test_day1() {
    let (part1, part2) = day1::day1(None).await;

    // Part 1
    assert_eq!(part1, 1603498);
    // Part 2
    assert_eq!(part2, 25574739);
}
