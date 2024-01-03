// https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b

// solution 1: 2333ms
use std::collections::HashMap;

fn part(n: i64) -> String {
    let val_products = products(n);
    let val_range = range(&val_products);
    let val_average = average(&val_products);
    let val_median = median(&val_products);
    let result = format!("Range: {} Average: {:.*} Median: {:.*}", val_range, 2, val_average, 2, val_median);
    result
}

fn partitions(n: i64) -> Vec<Vec<i64>> {
    let mut cache = HashMap::new();

    fn partitions_iter(n: i64, cache: &mut HashMap<i64, Vec<Vec<i64>>>) -> Vec<Vec<i64>> {
        if cache.contains_key(&n) {
            let ref result = cache[&n];
            return result.clone()
        } else {
            let mut result = vec![vec![n]];
            for i in 1..n {
                let sub_parts = partitions_iter(n - i, cache);
                for mut sub_part in sub_parts {
                    sub_part.push(i);
                    sub_part.sort();
                    result.push(sub_part);
                }
            }
            result.sort();
            result.dedup();
            cache.insert(n, result.clone());
            return result
        }
    }

    let result = partitions_iter(n, &mut cache);
    result
}

fn products(n: i64) -> Vec<i64> {
    let mut result = vec![];
    for item in partitions(n) {
        result.push(item.iter().product())
    }
    result.sort();
    result.dedup();
    result
}

fn range(ref r: &Vec<i64>) -> i64 {
    r[r.len() - 1] - r[0]
}

fn average(ref r: &Vec<i64>) -> f64 {
    let len = r.len();
    let sum: i64 = r.into_iter().sum();
    sum as f64 / len as f64
}

fn median(ref r: &Vec<i64>) -> f64 {
    let n = r.len();
    if n % 2 == 0 {
        return (r[n / 2] + r[n / 2 - 1]) as f64 / 2.0
    } else {
        return r[(n - 1) / 2] as f64
    }
}


