use crate::day7;
use std::fs;

#[tokio::test]
async fn test_day7_test_data() {
    let test_data = fs::read_to_string("src/day7/data/test_1.txt").unwrap();
    let (part1, part2) = day7::day7(Some(test_data)).await;

    // Part 1
    assert_eq!(part1, 3749);
    // Part 2
    assert_eq!(part2, 11387);
}

#[tokio::test]
async fn test_day7() {
    let (part1, part2) = day7::day7(None).await;

    // Part 1
    assert_eq!(part1, 5540634308362);
    // Part 2
    assert_eq!(part2, 472290821152397);
}
