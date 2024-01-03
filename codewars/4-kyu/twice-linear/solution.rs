// https://www.codewars.com/kata/5672682212c8ecf83e000050

// solution 1: 1111ms
use std::collections::VecDeque;
use std::cmp::min;

fn dbl_linear(n: u32) -> u32{
    let mut p2: VecDeque<u32> = VecDeque::new();
    let mut p3: VecDeque<u32> = VecDeque::new();
    let mut u = 1;
    
    for _ in 0..n {
        p2.push_back(u * 2 + 1);
        p3.push_back(u * 3 + 1);
        u = min(p2[0], p3[0]);
        if u == p2[0] {
            p2.pop_front();
        }
        if u == p3[0] {
            p3.pop_front();
        }
    }
    u
}


// solution 2: 1609ms
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dbl_linear(n: u32) -> u32{
    let mut heap = BinaryHeap::new();

    heap.push(Reverse(1));
    std::iter::from_fn(|| {
        let ret = heap.pop()?.0;
        while heap.peek().map(|&Reverse(x)| x == ret) == Some(true) {
            heap.pop();
        }

        heap.push(Reverse(ret * 2 + 1));
        heap.push(Reverse(ret * 3 + 1));
        Some(ret)
    })
    .nth(n as usize)
    .unwrap()
}