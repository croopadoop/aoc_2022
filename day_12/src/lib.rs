use std::collections::{HashMap, VecDeque};
mod helpers;
// const INPUT_START: &str = "./test_start.txt";
const INPUT: &str = "./input.txt";

pub fn process_part1() -> usize {
    // Get the grid from the input
    let mut grid = helpers::parse_to_char_array(INPUT);
    let shortest_route = match find_the_shortest_route(&mut grid) {
        Some(length) => length,
        None => 0 as usize,
    };

    shortest_route
}

fn find_the_shortest_route(grid: &mut Vec<Vec<char>>) -> Option<usize> {
    // Implementation of Breadth-First-Search Algorithm

    // Create a queue to store the coordinates to check
    let mut queue = VecDeque::new();

    // Create a hashmap that stores the coordinate on the grid, and the number of steps from the start required to get to that coordinate.
    let mut visited = HashMap::new();

    // Initialize start and end coordinates
    let mut start = (0, 0);
    let mut end = (0, 0);

    // Set max width and height for use in comparisons
    let max_height = grid.len() as i32;
    let max_width = grid[0].len() as i32;

    // Iterate over the grid to find start and end coordinates
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i, j);
                grid[i][j] = 'a'; // Replace 'S' with 'a' for easier comparison of height later.
            } else if grid[i][j] == 'E' {
                end = (i, j);
                grid[i][j] = 'z'; // Replace 'E' with 'z' for easier comparison of height later.
            }
        }
    }

    // Add the
    queue.push_back((start, 0));
    visited.insert(start, 0);

    // Iterate while the queue is not empty.
    while !queue.is_empty() {
        let (current, steps) = queue.pop_front().unwrap();
        let (i, j) = current;

        // Check if we have reached the end
        if current == end {
            return Some(steps);
        }

        // Iterate over the adjacent cardinal indices
        for (ii, jj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ai = i as i32 + ii;
            let aj = j as i32 + jj;

            // Only evaluate nodes that are on the grid
            if ai >= 0 && ai < max_height && aj >= 0 && aj < max_width {
                let next_step = (ai as usize, aj as usize);
                let current_elevation = grid[i][j] as u8;
                let next_elevation = grid[next_step.0][next_step.1] as u8;

                // Calculate the elevation distance by casting to an i8 so no subtraction overflow occurs.
                let elevation_difference = next_elevation as i8 - current_elevation as i8;

                // If the elevation difference is 1 or less, then it is a valid node to step to and should be added to the nodes
                // visited and to the queue for further evaluation of its neighbors.
                if elevation_difference <= 1 && !visited.contains_key(&next_step) {
                    visited.insert(next_step, steps + 1);
                    queue.push_back((next_step, steps + 1));
                }
            }
        }
    }

    None
}

fn find_the_shortest_route_pt2(
    grid: &mut Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    // Implementation of Breadth-First-Search Algorithm

    // Create a queue to store the coordinates to check
    let mut queue = VecDeque::new();

    // Create a hashmap that stores the coordinate on the grid, and the number of steps from the start required to get to that coordinate.
    let mut visited = HashMap::new();

    // Set max width and height for use in comparisons
    let max_height = grid.len() as i32;
    let max_width = grid[0].len() as i32;

    // Add the starting point to the queue and list of places visited
    queue.push_back((start, 0));
    visited.insert(start, 0);

    // Iterate while the queue is not empty.
    while !queue.is_empty() {
        let (current, steps) = queue.pop_front().unwrap();
        let (i, j) = current;

        // Check if we have reached the end
        if current == end {
            return Some(steps);
        }

        // Iterate over the adjacent cardinal indices
        for (ii, jj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ai = i as i32 + ii;
            let aj = j as i32 + jj;

            // Only evaluate nodes that are on the grid
            if ai >= 0 && ai < max_height && aj >= 0 && aj < max_width {
                let next_step = (ai as usize, aj as usize);
                let current_elevation = grid[i][j] as u8;
                let next_elevation = grid[next_step.0][next_step.1] as u8;

                // Calculate the elevation distance by casting to an i8 so no subtraction overflow occurs.
                let elevation_difference = next_elevation as i8 - current_elevation as i8;

                // If the elevation difference is 1 or less, then it is a valid node to step to and should be added to the nodes
                // visited and to the queue for further evaluation of its neighbors.
                if elevation_difference <= 1 && !visited.contains_key(&next_step) {
                    visited.insert(next_step, steps + 1);
                    queue.push_back((next_step, steps + 1));
                }
            }
        }
    }

    None
}

pub fn process_part2() -> usize {
    // Implement Breadth-First-Search for every node that begins with 'a'
    // Return the shortest route

    // Get the grid
    let mut grid = helpers::parse_to_char_array(INPUT);

    // Initialize a vector that will store the coordinates of each 'a' on the grid
    let mut starting_points: Vec<(usize, usize)> = Vec::new();

    // Initialize the start and end points
    let mut end = (0, 0);

    // Initialize a variable to store the shortest route
    let mut shortest_route = 0;

    // Iterate over the grid to find starting and end coordinates
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                starting_points.push((i, j));
                grid[i][j] = 'a'; // Replace 'S' with 'a' for easier comparison of height later.
            } else if grid[i][j] == 'E' {
                end = (i, j);
                grid[i][j] = 'z'; // Replace 'E' with 'z' for easier comparison of height later.
            } else if grid[i][j] == 'a' {
                starting_points.push((i, j)); // Add every point that is an 'a' to the list of potential starting points.
            }
        }
    }

    while !starting_points.is_empty() {

        // Calculate the shortest distance for the starting point.
        let shortest_distance_to_end = find_the_shortest_route_pt2(&mut grid, starting_points.pop().unwrap(), end);

        // If the distance to the end for this starting point is less than the shortest found so far, then
        // replace the shortest_route length with this starting point's distance.
        // Also replace the shortest_route if it is 0
        match shortest_distance_to_end {
            Some(distance) => {
                if distance < shortest_route || shortest_route == 0 {
                    shortest_route = distance;
                }
            }
            None => {
                0;
            }
        }
    }

    shortest_route
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_works() {
        let result = process_part1();
        assert_eq!(result, 31);
    }

    #[test]
    fn process_part2_works() {
        let result = process_part2();
        assert_eq!(result, 29);
    }
}
