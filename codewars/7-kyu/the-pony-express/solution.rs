// https://www.codewars.com/kata/5b18e9e06aefb52e1d0001e9

// solution 1: 1748ms
fn riders(stations: &Vec<u32>) -> u32 {
    let mut riders = 1;
    let mut miles = 0;
    
    for station in stations.iter().cloned() {
        miles += station;
        if miles > 100 {
            riders += 1;
            miles = station;
        }
    }
    
    riders
}


// solution 2: 1497ms
fn riders(stations: &Vec<u32>) -> u32 {
    stations
        .iter()
        .fold((0, 1), |(mile, rider), &x| if mile + x > 100 { (x, rider + 1) } else { (mile + x, rider) })
        .1
}