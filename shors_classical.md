# Simplified Shor's Algorithm in Rust (Educational)

## Description

This project provides a highly simplified, classical implementation of the order-finding portion of Shor's algorithm for educational purposes. It demonstrates the mathematical concepts behind the algorithm without implementing the quantum components.

**Important Note:** This is not a complete quantum implementation. It is a classical simulation of the order-finding part of Shor's algorithm, meant for educational understanding.

## Key Concepts Explained for Beginners

### GCD (Greatest Common Divisor)

* The `gcd` function calculates the largest number that divides both `a` and `b` without leaving a remainder.
* It uses Euclid's algorithm, which is an efficient way to find the GCD.

### Order-Finding

* The `find_order` function is the core of the classical part of Shor's algorithm.
* It finds the smallest positive integer `r` such that `a^r % n == 1`. This `r` is called the order of `a` modulo `n`.
* It checks if `a` and `n` are coprime (their GCD is 1). If not, it can't find the order.
* It calculates `a^r % n` iteratively and keeps track of the values it has seen. This is to avoid infinite loops if the order does not exist within the search range.
* It returns `Some(r)` if the order is found, or `None` if it's not.

### Simplified Shor's Algorithm

* The `simplified_shors_algorithm` function attempts to factor `n` using the order-finding function.
* It handles the case where `n` is even.
* It chooses a random number `a` between 2 and `n - 1`.
* It calculates the GCD of `a` and `n`. If it's not 1, it means `a` is a factor of `n`.
* It calls `find_order` to find the order `r` of `a` modulo `n`.
* If `r` is even, it calculates `x = a^(r/2) + 1` and `y = a^(r/2) - 1`.
* It calculates the GCD of `x` and `n` and the GCD of `y` and `n`. These GCDs are potential factors of `n`.
* It returns `Some((factor1, factor2))` if it finds two non-trivial factors, or `None` if it fails.
* **Note:** The real Shor's algorithm uses a quantum Fourier transform and quantum circuits. This is a classical simulation of the order-finding part.

### `main` Function

* The `main` function demonstrates how to use `simplified_shors_algorithm` to factor a number.
* It calls the function and prints the factors if found.

## Important Notes

* This is a highly simplified version of Shor's algorithm. The actual quantum algorithm uses quantum computers to perform the order-finding step much more efficiently.
* This code is not guaranteed to find factors for all numbers. It's primarily for educational purposes to illustrate the classical part of the algorithm.
* Real Shor's algorithm is exponentially faster than the best classical factoring algorithms.
* The number of tries in the `for` loop is a classical simulation of the probabilistic nature of the quantum part of Shor's algorithm.
