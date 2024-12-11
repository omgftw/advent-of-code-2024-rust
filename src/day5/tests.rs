use crate::day5;
use std::fs;

#[tokio::test]
async fn test_day5_part1_test_data() {
    let test_data = fs::read_to_string("src/day5/data/test_1.txt").unwrap();
    let (part1, _) = day5::day5(Some(test_data)).await;

    assert_eq!(part1, 143);
}

#[tokio::test]
async fn test_day5_part2_test_data() {
    let test_data = fs::read_to_string("src/day5/data/test_1.txt").unwrap();
    let (_, part2) = day5::day5(Some(test_data)).await;

    assert_eq!(part2, 1);
}

#[tokio::test]
async fn test_day5() {
    let (part1, part2) = day5::day5(None).await;

    // Part 1
    assert_eq!(part1, 1);
    // Part 2
    assert_eq!(part2, 1);
}
