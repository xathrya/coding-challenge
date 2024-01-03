// https://www.codewars.com/kata/540afbe2dc9f615d5e000425

// solution 1: 2233ms
use std::collections::HashSet;
use std::iter::FromIterator;
use std::f32;

fn vec_to_set(vec: Vec<u32>) -> HashSet<u32> {
    HashSet::from_iter(vec)
}

struct Sudoku{
    data: Vec<Vec<u32>>,
}

impl Sudoku{
    fn is_valid(&self) -> bool {
        let n = self.data.len() as u32;
        let basic_set: HashSet<u32> = HashSet::from_iter(1..n + 1);
        
        // validate row
        for row in self.data.clone() {
            if vec_to_set(row) != basic_set {
                return false;
            }
        }

        // validate column
        for i in 0..n {
            let mut col: Vec<u32> = vec![];
            for j in 0..n {
                col.push(self.data[j as usize][i as usize]);
            }
            if vec_to_set(col) != basic_set {
                return false;
            }
        }

        // validate square
        let length = n as f32;
        let size = length.sqrt() as u32;
        if size * size != n {
            return false;
        }
        for x in 0..size {
            for y in 0..size {
                let mut square: Vec<u32> = vec![];
                for i in (x * size)..(x * size + size) {
                    for j in (y * size)..(y * size + size) {
                        square.push(self.data[i as usize][j as usize]);
                    }
                }
                if vec_to_set(square) != basic_set {
                    return false;
                }
            }
        }
        true
    }
}