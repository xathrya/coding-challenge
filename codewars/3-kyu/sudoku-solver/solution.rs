// https://www.codewars.com/kata/5296bc77afba8baa690002d7

// solution 1: 2147ms
use std::collections::{HashMap, HashSet};

// Solve the given puzzle in place, no need to return a copy
fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let map: HashMap<(usize, usize), HashSet<u8>> = HashMap::new();

    fn inner(puzzle: &mut [[u8; 9]; 9], mut map: HashMap<(usize, usize), HashSet<u8>>) {
        if !puzzle.iter().any(|i| i.iter().any(|j| *j == 0)) {
            return;
        }
        for i in 0..9 {
            for j in 0..9 {
                if puzzle[i][j] == 0 {
                    let mut test = if let Some(s) = map.get(&(i, j)) {
                        s.clone()
                    } else {
                        (1..10).collect()
                    };
                    let mut other = HashSet::new();
                    let positions = get_related_position((i, j));
                    for p in positions {
                        other.insert(puzzle[p.0][p.1]);
                    }
                    test = test.difference(&other).copied().collect();
                    if test.len() == 1 {
                        puzzle[i][j] = *test.iter().next().unwrap();
                    } else {
                        map.insert((i, j), test);
                    }
                }
            }
        }
        let s = to_string(puzzle);
        println!("{}\n", s);
        inner(puzzle, map)
    }

    inner(puzzle, map)
}

fn to_string(puzzle: &[[u8; 9]; 9]) -> String {
    puzzle.iter()
        .map(|i| i.map(|j| j.to_string()).join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_related_position(c: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut positions = HashSet::new();
    for i in 0..9 {
        positions.insert((i, c.1));
        positions.insert((c.0, i));
    }
    let x1 = c.0 / 3 * 3;
    let y1 = c.1 / 3 * 3;
    for x in x1..(x1 + 3) {
        for y in y1..(y1 + 3) {
            positions.insert((x, y));
        }
    }
    positions.remove(&c);
    positions
}


// solution 2: 1919ms
use std::{
    collections::HashSet,
    iter::FromIterator,
};
fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let s: HashSet<u8> = HashSet::from_iter(1..=9);
    for r in 0..9 {
        for c in 0..9 {
            if puzzle[r][c] == 0 {
                let br = r / 3 * 3;
                let bc = c / 3 * 3;
                let block: HashSet<u8> = HashSet::from_iter(
                    (0..3_usize)
                        .flat_map(|row| (0..3_usize).map(move |col| (row, col)))
                        .map(|(r, c)| puzzle[br + r][bc + c]),
                );
                let row = HashSet::from(puzzle[r]);
                let col = HashSet::from_iter(puzzle.iter().map(|row| row[c]));
                let x = &s - &(&(&row | &col) | &block);
                if x.len() == 1 {
                    puzzle[r][c] = *x.iter().next().unwrap() as u8;
                    sudoku(puzzle)
                }
            }
        }
    }
}