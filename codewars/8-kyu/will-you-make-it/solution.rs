// https://www.codewars.com/kata/5861d28f124b35723e00005e

// solution 1: 1149ms
fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    distance_to_pump <= mpg * gallons 
}