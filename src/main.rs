use std::io::{self, BufRead};

fn max_toys(prices: &[usize], k: usize) -> usize {
    let mut sorted_prices = prices.to_vec();
    sorted_prices.sort();

    let mut count = 0;
    let mut total_spent = 0;

    for &price in &sorted_prices {
        if total_spent + price <= k {
            total_spent += price;
            count += 1;
        } else {
            break;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the total budget
    let total_budget: usize = stdin_iterator.next().unwrap().unwrap().trim().parse().expect("Invalid Integer");

    // Read the toy prices
    let toy_prices: Vec<usize> = stdin_iterator.next().unwrap().unwrap().trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Integer only"))
        .collect();

    // Get the maximum number of toys
    let max_toys_count = max_toys(&toy_prices, total_budget);

    println!("Maximum number of toys Mark can buy: {}", max_toys_count);
}
