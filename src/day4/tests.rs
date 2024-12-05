use crate::day4;
use std::fs;

#[tokio::test]
async fn test_day4_test_data() {
    let test_data = fs::read_to_string("src/day4/data/test_1.txt").unwrap();
    let (part1, part2) = day4::day4(Some(test_data)).await;

    // Part 1
    assert_eq!(part1, 18);
    // Part 2
    assert_eq!(part2, 9);
}

#[tokio::test]
async fn test_day4() {
    let (part1, part2) = day4::day4(None).await;

    // Part 1
    assert_eq!(part1, 2603);
    // Part 2
    assert_eq!(part2, 1965);
}
