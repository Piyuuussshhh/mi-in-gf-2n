use std::io::{self, Write};

mod polynomial;
use polynomial::irr_poly::get_irreducible_polynomial;
use polynomial::object;
use polynomial::validation::validate;

fn main() {
    // Take the polynomial who's multiplicative inverse is to be found as input.
    let mut input_poly_coeffs = String::new();
    println!(
        "Enter the coefficients of the polynomial (highest to lowest, eg 1101 for x^3 + x^2 + 1):"
    );
    io::stdin()
        .read_line(&mut input_poly_coeffs)
        .expect("Error collecting coefficients");
    let input_poly_coeffs = input_poly_coeffs.trim();
    if input_poly_coeffs.chars().any(|v| v != '1' && v != '0') {
        panic!("Only input 1s and 0s");
    }

    // Convert the bit string to a terms array.
    let input_poly_terms = input_poly_coeffs
        .chars()
        .enumerate()
        .filter_map(|(idx, bit)| {
            if bit == '1' {
                let term_power = (input_poly_coeffs.len() - 1 - idx) as u8;
                Some(term_power)
            } else {
                None
            }
        })
        .collect::<Vec<u8>>();

    // The N in GF(2^N).
    let n: u32 = input_poly_coeffs.len() as u32;
    let input_poly = object::GF2NPolynomial::new(input_poly_terms);
    println!("Input polynomial: {}", input_poly.algebraic_string());

    println!(
        "The first Galois Field the polynomial belongs to is GF(2^{n}), therefore we will select a suitable irreducible polynomial"
    );

    // Calculate and get the first irreducible polynomial of degree n over GF(2^n).
    let irr_poly = get_irreducible_polynomial(n);
    if irr_poly == object::GF2NPolynomial::zero() {
        println!("Could not find an irreducible polynomial. Exiting...");
        return;
    }

    // Calculate inverse using extended euclidean algorithm.
    let inverse_poly = input_poly.inverse(&irr_poly);
    println!(
        "The inverse of {} with respect to the irreducible polynomial {} in GF(2^{}): {}",
        input_poly.algebraic_string(),
        irr_poly.algebraic_string(),
        n,
        inverse_poly.algebraic_string(),
    );

    io::stdout().flush().expect("err");
    print!("\nPress enter to validate the output...");
    io::stdout().flush().expect("err");
    let mut enter = String::new();
    io::stdin()
        .read_line(&mut enter)
        .expect("Could not take enter input");

    validate(&input_poly, &inverse_poly, &irr_poly, n);
}
