use std::collections::VecDeque;

type Point = (usize, usize);

pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: Point,
    end: Point,
) -> Vec<Point> {
    let height = maze.len();
    let width = maze[0].len(); // Assuming all rows are the same length.
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;
    
    if start_x >= height || start_y >= width || end_x >= height || end_y >= width {
        println!("Start or end is outside the maze!");
        return vec![];
    }

    if maze[start_x][start_y] != 'S' || maze[end_x][end_y] != 'E' {
        println!("Invalid start or end position!");
        return vec![];
    }

    let mut queue = VecDeque::<Vec<Point>>::new();
    queue.push_back(vec![start]);

    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let (x, y) = path[path.len() - 1];

        if (x, y) == end {
            // Found solution.
            return path;
        }

        let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        for (dx, dy) in directions {
            let (next_x, next_y) = (x as i32 + dx, y as i32 + dy);
            if next_x < 0 || next_y < 0 {
                continue; // Outside the maze.
            }
            let (next_x, next_y) = (next_x as usize, next_y as usize);
            if next_x >= height || next_y >= width {
                continue; // Outside the maze.
            }
            if maze[next_x][next_y] != '#' && !path.contains(&(next_x, next_y)) {
                let mut new_path = path.clone();
                new_path.append(&mut vec![(next_x, next_y)]);
                queue.push_back(new_path);
            }
        }
    }
    Vec::<Point>::new()
}

