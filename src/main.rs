use std::io;

mod polynomial;
use polynomial::object;

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
    if let Some('0') = input_poly_coeffs.chars().nth(0) {
        panic!("Most significant bit (MSB) has to be set, else its redundant")
    }
    let input_poly_coeffs = input_poly_coeffs
        .chars()
        .map(|v| if v == '1' { true } else { false })
        .collect::<Vec<bool>>();
    let input_poly = object::Polynomial {
        degree: (input_poly_coeffs.len() - 1) as u32,
        coeffs: input_poly_coeffs,
    };
    println!("Input polynomial: {}", input_poly.algebraic_string());

    // The N in GF(2^N).
    let n: u32 = input_poly.degree;
    println!("The first Galois Field the polynomial belongs to is GF(2^{n}), therefore we will select a suitable irreducible polynomial");

    // Calculate and get the first irreducible polynomial of degree n over GF(2^n).
    let irr_poly = get_irreducible_polynomial(n);
    if irr_poly == object::Polynomial::zero() {
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
}
