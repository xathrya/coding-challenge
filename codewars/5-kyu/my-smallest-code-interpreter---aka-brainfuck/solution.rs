// https://www.codewars.com/kata/526156943dfe7ce06200063e

// solution 1: 2042ms
use std::collections::HashMap;
use std::num::Wrapping;

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let src: Vec<char> = code.chars().collect();
    let brackets: HashMap<usize, usize> = {
        let mut m = HashMap::new();
        let mut scope_stack = Vec::new();
        for (idx, ch) in src.iter().enumerate() {
            match ch {
                &'[' => {
                    scope_stack.push(idx);
                },
                &']' => {
                    m.insert(scope_stack.pop().unwrap(), idx);
                },
                _ => {},
            }
        }
        m
    };

    let mut pc: usize = 0;                                      // Program counter
    let mut mem: [Wrapping<u8>; 500] = [Wrapping(0); 500];      // Program memeory
    let mut ptr: usize = 0;                                     // Pointer
    let mut stack: Vec<usize> = Vec::new();                     // Bracket stack
    let mut result = vec![];
    let mut reader = input.into_iter();

    while pc < src.len() {
        let Wrapping(val) = mem[ptr];

        const ONE: Wrapping<u8> = Wrapping(1);
        match src[pc] {
            '>' => { ptr = (ptr + 1) % 256; },
            '<' => { ptr = (ptr + 255) % 256; },
            '+' => { mem[ptr] = mem[ptr] + ONE; },
            '-' => { mem[ptr] = mem[ptr] - ONE; },
            '.' => { result.push(val); },
            ',' => { mem[ptr] = Wrapping(reader.next().unwrap()); },
            '[' => {
                if val == 0 {
                    pc = brackets[&pc];
                } else {
                    stack.push(pc);
                }
            },
            ']' => {
                let matching_bracket = stack.pop().unwrap();
                if val != 0 {
                    pc = matching_bracket - 1;
                }
            },
            _ => {},
        }
        pc += 1;
    }
    result
}

