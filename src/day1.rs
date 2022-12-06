pub fn calories_per_elf(input: &str) -> Vec<i32> {
    let mut sum = 0;
    let mut calories = Vec::new();
    let test = input.split("\n").collect::<Vec<_>>();

    for s in test {
        if s == "" {
            calories.push(sum);
            sum = 0;
            continue;
        }

        let value = s.parse::<i32>().unwrap();

        sum = sum + value;
    }

    calories.sort_by(|a, b| b.cmp(a));

    return calories;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_input() {
        let test_input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let calories = calories_per_elf(test_input);

        let most_calories = calories.first().unwrap();

        assert_eq!(most_calories, &24000)
    }

    #[test]
    fn real_input() {
        use std::fs;

        let input = fs::read_to_string("./src/day1_input.txt").unwrap();

        let calories = calories_per_elf(&input);

        let most_calories = calories.first().unwrap();
        println!("{}", most_calories);

        assert_eq!(most_calories, &70764);
    }

    #[test]
    fn part_two() {
        use std::fs;

        let input = fs::read_to_string("./src/day1_input.txt").unwrap();

        let calories = calories_per_elf(&input);

        let first_three = &calories[0..3];
        let mut sum: i32 = 0;

        for num in first_three {
            sum = sum + num;
        }

        assert_eq!(sum, 203905);
    }
}
