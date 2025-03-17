// Shor's Algorithm (Simplified for Educational Purposes)
// This is a highly simplified version and does not implement the full quantum algorithm.
// It focuses on the classical part of the algorithm, which is the order-finding.

use rand::Rng;
use std::collections::HashSet;

// Function to calculate the greatest common divisor (GCD) using Euclid's algorithm.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to find the order of a modulo n.
fn find_order(a: u64, n: u64) -> Option<u64> {
    if gcd(a, n) != 1 {
        return None; // a and n must be coprime.
    }

    let mut r = 1;
    let mut x = a % n;
    let mut seen = HashSet::new(); //To prevent infinite loops with bad inputs

    while x != 1 {
        if seen.contains(&x) {
            return None; //Order not found.
        }
        seen.insert(x);
        x = (x * a) % n;
        r += 1;
        if r > n {
            return None; //Order not found.
        }
    }
    Some(r)
}

// Simplified Shor's algorithm for factoring n.
fn simplified_shors_algorithm(n: u64) -> Option<(u64, u64)> {
    if n % 2 == 0 {
        return Some((2, n / 2)); // Handle even numbers.
    }

    let mut rng = rand::thread_rng();

    for _ in 0..100 { // Try a few times. Real Shor's needs far more tries.
        let a = rng.gen_range(2..n); // Choose a random a between 2 and n-1.

        let g = gcd(a, n);
        if g != 1 {
            return Some((g, n / g)); // Found a factor.
        }

        if let Some(r) = find_order(a, n) {
            if r % 2 == 0 {
                let x = (a.pow(r as u32 / 2) + 1) % n;
                let y = (a.pow(r as u32 / 2) - 1) % n;

                let factor1 = gcd(x, n);
                let factor2 = gcd(y, n);

                if factor1 != 1 && factor1 != n && factor2 != 1 && factor2 != n {
                    return Some((factor1, factor2));
                }
            }
        }
    }
    None // Factoring failed.
}

fn main() {
    let n = 297; // Number to factor.
    if let Some((factor1, factor2)) = simplified_shors_algorithm(n) {
        println!("Factors of {}: {} and {}", n, factor1, factor2);
    } else {
        println!("Factoring failed.");
    }

    let n2 = 356;
    if let Some((factor1, factor2)) = simplified_shors_algorithm(n2) {
        println!("Factors of {}: {} and {}", n2, factor1, factor2);
    } else {
        println!("Factoring failed.");
    }

    let n3 = 216;
    if let Some((factor1, factor2)) = simplified_shors_algorithm(n3) {
        println!("Factors of {}: {} and {}", n3, factor1, factor2);
    } else {
        println!("Factoring failed.");
    }
}
