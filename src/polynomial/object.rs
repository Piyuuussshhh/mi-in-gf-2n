#![allow(non_snake_case)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial {
    pub degree: u32,
    // If true, bit at that index is set (1). Else if false, bit at that index is not set (0).
    pub coeffs: Vec<bool>,
}

impl Polynomial {
    pub fn zero() -> Self {
        Self {
            degree: 0,
            coeffs: vec![false],
        }
    }

    fn one() -> Self {
        Self {
            degree: 0,
            coeffs: vec![true],
        }
    }

    pub fn trim(mut self) -> Self {
        if let Some(first_one) = self.coeffs.iter().position(|&c| c) {
            self.coeffs = self.coeffs.split_off(first_one);
            self.degree = (self.coeffs.len() - 1) as u32;
        } else {
            // This is the zero polynomial
            self.coeffs = vec![false];
            self.degree = 0;
        }
        self
    }

    pub fn algebraic_string(&self) -> String {
        if self == Polynomial::zero() {
            println!("0");
            return;
        }

        let poly_clone = self.clone();
        let poly_clone = poly_clone
            .coeffs
            .into_iter()
            .enumerate()
            .map(|(idx, bit)| {
                if bit {
                    let power = poly_clone.degree as usize - idx;
                    match power {
                        0 => "1".to_string(),
                        1 => "x".to_string(),
                        _ => format!("x^{}", power),
                    }
                } else {
                    "".to_string()
                }
            })
            .filter(|term| !term.is_empty())
            .collect::<Vec<String>>();

        poly_clone.join(" + ")
    }

    fn divide_by(&self, divisor: &Polynomial) -> (Self, Self) {
        let dividend_len = (self.degree + 1) as usize;
        let divisor_len = (divisor.degree + 1) as usize;

        // If the divisor is greater than the dividend, quotient = 0, remainder = dividend. eg 3 / 5 -> (0, 3)
        if dividend_len < divisor_len || self.degree < divisor.degree {
            return (Self::zero(), self.clone());
        }

        /*
            Think about it.
            eg. If you divide some polynomial p(x) = x^6 + ... with some other polynomial g(x) = x^2 + ...,
            the degree of the quotient will be 6 - 2 = 4 -> (dividend's degree - divisor's degree = h, let's say)

            Then the length of a bit string to represent a polynomial of degree n will always be h + 1.
        */
        let mut quotient = Self {
            degree: self.degree - divisor.degree,
            coeffs: vec![false; (dividend_len - divisor_len) + 1],
        };

        let mut remainder = self.clone();

        for i in 0..=(dividend_len - divisor_len) {
            if remainder.coeffs[i] {
                quotient.coeffs[i] = true;
                for j in 0..divisor_len {
                    remainder.coeffs[i + j] ^= divisor.coeffs[j];
                }
            }
        }

        // Remove the leading 0s (false) from the remainder and set its degree.
        let remainder = remainder.trim();

        (quotient, remainder)
    }

    /// Calculated using the Extended Euclidean Algorithm.
    pub fn inverse(&self, irreducible_polynomial: &Polynomial) -> Self {
        let mut Q = Polynomial::zero();
        let mut A = self.clone();
        let mut B = irreducible_polynomial.clone();
        let mut R = Polynomial::one();
        let mut T1 = Polynomial::zero();
        let mut T2 = Polynomial::one();
        let mut S = Polynomial::zero();

        while R != Polynomial::zero() {
            
        }
    }
}