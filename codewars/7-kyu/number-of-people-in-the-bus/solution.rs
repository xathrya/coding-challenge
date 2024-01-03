// https://www.codewars.com/kata/5648b12ce68d9daa6b000099

// solution 1: 1160ms
fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops.iter().fold(0, |acc,x| acc + x.0 - x.1)
}