fn main() {
    let input = include_str!("./input.txt").lines().collect::<Vec<_>>();

    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    for line in input {
        let test: Vec<i64> = line.split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
        //Part 1
        sum1 += next_element(&test);
        //Part 2
        sum2 += next_element(&test.into_iter().rev().collect())
    }
    println!("Part 1: {sum1}\nPart 2: {sum2}")
}

fn next_element(series: &Vec<i64>) -> i64 {
    if series.iter().filter(|&&x| x != 0).count() == 0 {
        return 0
    }
    else {
        let differences: Vec<i64> = series.windows(2)
        .map(|window| window[1] - window[0])
        .collect();
        return series.last().unwrap() + next_element(&differences);
    }
}