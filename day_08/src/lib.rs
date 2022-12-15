mod helpers;
const INPUT: &str = "./input.txt";

pub fn process_part1() -> i32 {
    let trees = helpers::parse_to_digit_array(INPUT);
    let mut count_visible = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            // Count the edge trees
            if (i == 0) || (j == 0) || (i == trees.len() - 1) || (j == trees[i].len() - 1) {
                count_visible += 1;
            } else {
                let tree_height = trees[i][j];
                let idx_j = j.clone();
                let idx_i = i.clone();

                let mut visible_left = true;
                let mut visible_right = true;
                let mut visible_up = true;
                let mut visible_down = true;

                // check visibility left
                for l in 0..idx_j {
                    if visible_left && tree_height > trees[i][l] {
                        continue;
                    } else {
                        visible_left = false;
                    }
                }

                if visible_left {
                    count_visible += 1;
                    continue;
                }

                // check visibility right
                for r in idx_j + 1..trees[i].len() {
                    if visible_right && tree_height > trees[i][r] {
                        continue;
                    } else {
                        visible_right = false;
                    }
                }

                if visible_right {
                    count_visible += 1;
                    continue;
                }

                // check visiblity up
                for u in 0..idx_i {
                    if visible_up && tree_height > trees[u][j] {
                        continue;
                    } else {
                        visible_up = false;
                    }
                }

                if visible_up {
                    count_visible += 1;
                    continue;
                }

                // check visibility down
                for d in idx_i + 1..trees.len() {
                    if visible_down && tree_height > trees[d][j] {
                        continue;
                    } else {
                        visible_down = false;
                    }
                }

                if visible_down {
                    count_visible += 1;
                    continue;
                }
            }
        }
    }

    count_visible
}

pub fn process_part2() -> i32 {
    let trees = helpers::parse_to_digit_array(INPUT);
    let mut max_scenic_score = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            if (i == 0) || (j == 0) || (i == trees.len() - 1) || (j == trees[i].len() - 1) {
                continue;
            } else {
                // Calc Scenic Score
                let tree_height = trees[i][j];
                let range_i = 0..i;
                let range_j = 0..j;

                let mut score_left = 0;
                let mut score_right = 0;
                let mut score_up = 0;
                let mut score_down = 0;

                // check visibility left
                for l in range_j.rev() {
                    if tree_height > trees[i][l] {
                        score_left += 1;
                    } else {
                        score_left += 1;
                        break;
                    }
                }

                for r in j + 1..trees[i].len() {
                    if tree_height > trees[i][r] {
                        score_right += 1;
                    } else {
                        score_right += 1;
                        break;
                    }
                }

                // check visiblity up
                for u in range_i.rev() {
                    if tree_height > trees[u][j] {
                        score_up += 1;
                    } else {
                        score_up += 1;
                        break;
                    }
                }

                // check visibility down
                for d in i + 1..trees.len() {
                    if tree_height > trees[d][j] {
                        score_down += 1;
                    } else {
                        score_down += 1;
                        break;
                    }
                }

                let tree_score = score_left * score_right * score_up * score_down;

                if tree_score > max_scenic_score {
                    max_scenic_score = tree_score;
                }
            }
        }
    }
    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_works() {
        let result = process_part1();
        assert_eq!(result, 21);
    }

    #[test]
    fn process_part2_works() {
        let result = process_part2();
        assert_eq!(result, 8);
    }
}
