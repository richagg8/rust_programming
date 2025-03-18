use num_complex::Complex;
use std::f64::consts::PI;

fn dft(input: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let n = input.len();
    let mut output = vec![Complex::new(0.0, 0.0); n];

    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for t in 0..n {
            let angle = -2.0 * PI * (k as f64) * (t as f64) / (n as f64);
            let w = Complex::new(angle.cos(), angle.sin());
            sum += input[t] * w;
        }
        output[k] = sum;
    }

    output
}

fn main() {
    let input = vec![
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
    ];

    let output = dft(&input);

    println!("Input: {:?}", input);
    println!("DFT Output: {:?}", output);

    let input2 = vec![
        Complex::new(1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
        Complex::new(4.0, 0.0),
    ];

    let output2 = dft(&input2);

    println!("Input: {:?}", input2);
    println!("DFT Output: {:?}", output2);
}
