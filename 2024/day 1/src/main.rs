use itertools::Itertools;

fn convert(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
    .lines()
    .map(|line| {
        line.split_ascii_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap()
    })
    .unzip()
}

fn total_distance(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
    .iter()
    .sorted()
    .zip(right_list.iter().sorted())
    .map(|(x1, x2)| x1.abs_diff(*x2))
    .sum()
}

fn similarity_score(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
    .iter()
    .map(|left_number| {let score = right_list
        .iter()
        .filter(|right_number| *right_number == left_number)
        .count();
        left_number * u32::try_from(score).unwrap()
    })
    .sum()

}



fn main() {
    const DATA: &str = include_str!("../data/day1.txt");
    let (left_list, right_list) = convert(DATA);
    let distance = total_distance(&left_list, &right_list);
    println!("{}", distance);
    let similarity = similarity_score(&left_list, &right_list);
    println!("{}", similarity);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = include_str!("../data/test.txt");
    #[test]
    fn test_distance() {
        let (left_list, right_list) = convert(DATA);
        assert_eq!(total_distance(&left_list, &right_list), 11);
    }
    #[test]
    fn test_similarity(){
        let (left_list, right_list) = convert(DATA);
        assert_eq!(similarity_score(&left_list, &right_list), 31);
    }
}