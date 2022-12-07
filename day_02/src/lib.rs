    // A = Rock
    // B = Paper
    // C = Scissors

    // X = Rock
    // Y = Paper
    // Z = Scissors

    // Rock > Scissors (A > Z) (X > C)
    // Paper > Rock (B > X) (Y > A)
    // Scissors > Paper (C > Y) (Z > B)

    // Loss
    // (A Z) (B X) (C Y)

    // Win
    // (C X) (A Y) (B Z)

    // Draw
    // (A X) (B Y) (C Z) 

    // Scoring
    // 1 = Rock
    // 2 = Paper
    // 3 = Scissors
    // 0 = Loss
    // 3 = Draw
    // 6 = Win


pub fn process_part1(input: &str) -> String {
    println!("{:?}", input);
    let rock_paper_scissors_games = input.split("\r\n").collect::<Vec<_>>();
    let mut total_score = 0;
    for round in rock_paper_scissors_games {
        let round_point = match round {
            "C X" | "A Y" | "B Z" => 6, // Win
            "A Z" | "B X" | "C Y" => 0, // Loss
            _ => 3                      // Tie
        };

        let play = round.chars().last().unwrap();
        let play_point = match play {
            'X' => 1, // Rock
            'Y' => 2, // Paper
            'Z' => 3, // Scissors
            _ => 0
        };

        let sum = round_point + play_point;
        total_score += sum;
    }

    total_score.to_string()
}

    // A = Rock
    // B = Paper
    // C = Scissors

    // X = Lose
    // Y = Draw
    // Z = Win

    // Loss
    // (A C) (B A) (C B)

    // Win
    // (C A) (A B) (B C)

    // Draw
    // (A A) (B B) (C C) 

    // Scoring
    // 1 = Rock
    // 2 = Paper
    // 3 = Scissors
    // 0 = Loss
    // 3 = Draw
    // 6 = Win

pub fn process_part2(input: &str) -> String {
    let rock_paper_scissors_games = input.split("\r\n").collect::<Vec<_>>();
    let mut total_score = 0;

    for round in rock_paper_scissors_games {
        let round_point = match round {
            "A X" => 3, // Scissor Loss
            "A Y" => 4,
            "A Z" => 8, // Paper Win
            "B X" => 1, // Rock Loss
            "B Y" => 5,
            "B Z" => 9, // Scissor Win
            "C X" => 2, // Paper Loss
            "C Y" => 6,
            "C Z" => 7, // Rock Win
            _ => 0
        };

        total_score += round_point;
    }

    total_score.to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\r\nB X\r\nC Z";

    #[test]
    fn part1_works() {
        let result = process_part1(&INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(&INPUT);
        assert_eq!(result, "12");
    }
}
