enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn as_str(&self) -> &'static str {
        match self {
            RPS::Rock => "Rock",
            RPS::Paper => "Paper",
            RPS::Scissors => "Scissors",
        }
    }
}

fn code_to_rps(code: &str) -> RPS {
    return match code {
        "A" | "X" => RPS::Rock,
        "B" | "Y" => RPS::Paper,
        "C" | "Z" => RPS::Scissors,
        _ => RPS::Rock,
    };
}

fn calcluate_correct_move(opponent: &RPS, me: &str) -> RPS {
    return match (opponent, me) {
        (&RPS::Rock, "X") => RPS::Scissors,
        (&RPS::Rock, "Y") => RPS::Rock,
        (&RPS::Rock, "Z") => RPS::Paper,
        (&RPS::Paper, "X") => RPS::Rock,
        (&RPS::Paper, "Y") => RPS::Paper,
        (&RPS::Paper, "Z") => RPS::Scissors,
        (&RPS::Scissors, "X") => RPS::Paper,
        (&RPS::Scissors, "Y") => RPS::Scissors,
        (&RPS::Scissors, "Z") => RPS::Rock,
        _ => RPS::Rock,
    };
}

fn score_round(opponent: RPS, me: RPS) -> i32 {
    let mut score: i32 = 0;

    match me {
        RPS::Rock => score += 1,
        RPS::Paper => score += 2,
        RPS::Scissors => score += 3,
    }

    match (opponent, me) {
        (RPS::Rock, RPS::Rock) => score += 3,
        (RPS::Rock, RPS::Paper) => score += 6,
        (RPS::Rock, RPS::Scissors) => score += 0,
        (RPS::Scissors, RPS::Rock) => score += 6,
        (RPS::Scissors, RPS::Paper) => score += 0,
        (RPS::Scissors, RPS::Scissors) => score += 3,
        (RPS::Paper, RPS::Rock) => score += 0,
        (RPS::Paper, RPS::Paper) => score += 3,
        (RPS::Paper, RPS::Scissors) => score += 6,
    }

    return score;
}

pub fn do_some_rps(input: &str, part: i32) -> i32 {
    let rounds = input.lines();
    let mut score: i32 = 0;

    for round in rounds {
        let mut iter = round.splitn(2, " ");
        let opponent_code = iter.next().unwrap();
        let me_code = iter.next().unwrap();
        let opponent = code_to_rps(opponent_code);
        let me = if part == 1 {
            code_to_rps(me_code)
        } else {
            calcluate_correct_move(&opponent, me_code)
        };

        // println!(
        //     "code: {}, me: {}, oppononent: {}",
        //     me_code,
        //     me.as_str(),
        //     opponent.as_str()
        // );

        let round_score = score_round(opponent, me);

        score += round_score;
    }

    return score;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::do_some_rps;

    #[test]
    fn small_input_part_one() {
        let input = "A Y
B X
C Z";

        let score = do_some_rps(input, 1);

        assert_eq!(score, 15);
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("./src/day2_input.txt").unwrap();

        let score = do_some_rps(&input, 1);

        assert_eq!(score, 14827);
    }

    #[test]
    fn small_input_part_two() {
        let input = "A Y
B X
C Z";

        let score = do_some_rps(input, 2);

        assert_eq!(score, 12);
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("./src/day2_input.txt").unwrap();

        let score = do_some_rps(&input, 2);

        assert_eq!(score, 13889);
    }
}
