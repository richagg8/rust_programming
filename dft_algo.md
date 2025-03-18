# Discrete Fourier Transform (DFT) in Rust

This document explains a basic Rust implementation of the Discrete Fourier Transform (DFT), designed for beginners.

## Explanation and Key Improvements for Beginners

### `use num_complex::Complex;`

* This line imports the `Complex` type from the `num_complex` crate. The Discrete Fourier Transform (DFT) works with complex numbers, so this import is essential.

### `use std::f64::consts::PI;`

* Imports the constant `PI`, which is needed for the DFT's angular calculations.

### `fn dft(input: &[Complex<f64>]) -> Vec<Complex<f64>>`

* This defines the `dft` function, which takes a slice of complex numbers (`&[Complex<f64>]`) as input and returns a `Vec<Complex<f64>>` (a vector of complex numbers) as output.
* `&[Complex<f64>]` means a slice, which avoids unnecessary copying of the input data, making it more efficient.

### `let n = input.len();`

* Gets the length of the input signal, which is necessary for the DFT calculations.

### `let mut output = vec![Complex::new(0.0, 0.0); n];`

* Creates a vector `output` to store the DFT results. It's initialized with `n` complex numbers, all set to `0.0 + 0.0i`.

### Nested Loops

* The outer loop (`for k in 0..n`) iterates through the frequency bins.
* The inner loop (`for t in 0..n`) iterates through the time samples.

### `let angle = ...;`

* Calculates the angle for the complex exponential, which is a crucial part of the DFT formula.

### `let w = Complex::new(angle.cos(), angle.sin());`

* Creates the complex exponential (twiddle factor) using the cosine (`cos`) and sine (`sin`) of the calculated angle.

### `sum += input[t] * w;`

* Performs the core DFT calculation: multiplying the input sample by the twiddle factor and adding it to the `sum`.

### `output[k] = sum;`

* Stores the result for the current frequency bin in the `output` vector.

### `main()` function

* Provides example usages of the `dft` function.
* Prints the input and output vectors so you can see the results of the DFT.

### `num-complex` crate

* Remember to add the `num-complex` crate to your `Cargo.toml` file:

    ```toml
    [dependencies]
    num-complex = "0.4"
    ```

## How to Run This Code

1.  **Save the code:** Save the Rust code as a `.rs` file (e.g., `dft.rs`).
2.  **Create `Cargo.toml`:** Create a `Cargo.toml` file in the same directory with the `num-complex` dependency specified as shown above.
3.  **Run `cargo run`:** Open your terminal in the same directory as the files and run the command `cargo run`. This will compile and execute the Rust code.
