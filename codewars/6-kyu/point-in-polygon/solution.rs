// https://www.codewars.com/kata/530265044b7e23379d00076a

// solution 1: 1423ms
type Point = (f32, f32);

fn point_in_poly(poly: &[Point], (x, y): Point) -> bool {
    poly.iter()
        .zip(poly.iter().cycle().skip(1))
        .filter(|&(&(x1, y1), &(x2, y2))| {
            // check if the point is within the edge
            (y1 > y) != (y2 > y) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1)
        })
        .count() & 1 == 1
}


// solution 2: 1522ms
use itertools::Itertools;

type Point = (f32, f32);

fn point_in_poly(poly: &[Point], point: Point) -> bool {
    let mut result = false;
    let (x, y) = point;

    for (&(x1, y1), &(x2, y2)) in poly.into_iter().circular_tuple_windows() {
        if (y1 > y) != (y2 > y) && (x < x1 + (x2 - x1) * (y - y1) / (y2 - y1)) {
            result = !result;
        }
    }

    result
}