// https://www.codewars.com/kata/5765870e190b1472ec0022a2

// solution 1: 3162ms
#[derive(Clone, Copy, PartialEq)]
enum Road {
    Empty,
    Wall,
}

impl Road {
    fn from_char(c: char) -> Self {
        match c {
            'W' => Road::Wall,
            _ => Road::Empty,
        }
    }
}

type Maze = Vec<Vec<Road>>;

fn path_finder(maze: &str) -> bool {
    let maze_vec: Maze = maze
        .split_whitespace()
        .map(|s| s.chars().map(|c| Road::from_char(c)).collect())
        .collect();
    path_finder_recursive(&maze_vec, (0, 0), &mut vec![])
}

fn path_finder_recursive(maze: &Maze, position: (usize, usize), passed: &mut Vec<(usize, usize)>) -> bool {
    let (row, column) = (maze.len() - 1, maze[0].len() - 1);
    if position == (row, column) {
        return true;
    }
    let (x, y) = position;
    let north = (x, if y > 0 { y - 1 } else { 0 });
    let east = (if x < column { x + 1 } else { column }, y);
    let south = (x, if y < row { y + 1 } else { row });
    let west = (if x > 0 { x - 1 } else { 0 }, y);
    let mut directions = vec![north, east, south, west];
    directions.dedup();
    let avaiable_positions: Vec<(usize, usize)> = directions
        .into_iter()
        .filter(|p| !passed.contains(p) && p != &position)
        .filter(|(a, b)| maze[*a][*b] != Road::Wall)
        .collect();

    if avaiable_positions.len() <= 0 {
        return false;
    }

    avaiable_positions.iter().any(|p| {
        passed.push(position);
        passed.push(*p);
        path_finder_recursive(maze, *p, passed)
    })
}


// solution 2: 2175ms
fn path_finder(maze: &str) -> bool {
    let mut maze = maze 
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    check_path(&mut maze, (0, 0))
}

fn check_path(maze: &mut [Vec<char>], pos @ (x, y): (usize, usize)) -> bool {
    let max_n = maze.len() - 1;
    if pos == (max_n, max_n) {
        true 
    } else if let 'W' | 'B' = maze[x][y] {
        false 
    } else {
        maze[x][y] = 'B';
           x != 0     && check_path(maze, (x - 1, y))
        || x != max_n && check_path(maze, (x + 1, y))
        || y != 0     && check_path(maze, (x, y - 1))
        || y != max_n && check_path(maze, (x, y + 1))
    }
}


// solution 3: 2402ms
fn path_finder(maze: &str) -> bool {
    let mut maze: Vec<Vec<_>> = maze 
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let n = maze.len();
    let mut frontier: Vec<(usize, usize)> = vec![(0, 0)];
    
    loop {
        match frontier.pop() {
            None    => break,
            Some((x, y))    => {
                if maze[x][y] == '.' {
                    maze[x][y] = 'V';
                    if x > 0    { frontier.push((x - 1, y)) }
                    if y > 0    { frontier.push((x, y - 1)) }
                    if x < n-1  { frontier.push((x + 1, y)) }
                    if y < n-1  { frontier.push((x, y + 1)) }
                }
            }
        }
    }
    maze[n - 1][n - 1] == 'V'
}