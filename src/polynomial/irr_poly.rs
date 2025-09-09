use super::object::Polynomial;

pub fn get_irreducible_polynomial(n: u32) -> Polynomial {
    match n {
        1 => {
            return Polynomial {
                degree: 1,
                // p(x) = x
                coeffs: vec![true, false],
            };
        }
        2 => {
            return Polynomial {
                degree: 2,
                // p(x) = x^2 + x + 1
                coeffs: vec![true, true, true],
            };
        }
        3 => {
            return Polynomial {
                degree: 3,
                // p(x) = x^3 + x + 1
                coeffs: vec![true, false, true, true],
            };
        }
        4 => {
            return Polynomial {
                degree: 4,
                // p(x) = x^4 + x + 1
                coeffs: vec![true, false, false, true, true],
            };
        }
        5 => {
            return Polynomial {
                degree: 5,
                // p(x) = x^5 + x^2 + 1
                coeffs: vec![true, false, false, true, false, true],
            };
        }
        6 => {
            return Polynomial {
                degree: 6,
                // p(x) = x^6 + x + 1
                coeffs: vec![true, false, false, false, false, true, true],
            };
        }
        7 => {
            return Polynomial {
                degree: 7,
                // p(x) = x^7 + x + 1
                coeffs: vec![true, false, false, false, false, false, true, true],
            };
        }
        8 => {
            return Polynomial {
                degree: 8,
                // p(x) = x^8 + x^4 + x^3 + x + 1
                coeffs: vec![true, false, false, false, true, true, false, true, true],
            };
        }
        9 => {
            return Polynomial {
                degree: 9,
                // p(x) = x^9 + x + 1
                coeffs: vec![
                    true, false, false, false, false, false, false, false, true, true,
                ],
            };
        }
        10 => {
            return Polynomial {
                degree: 10,
                // p(x) = x^10 + x^3 + 1
                coeffs: vec![
                    true, false, false, false, false, false, false, true, false, false, true,
                ],
            };
        }
        16 => {
            return Polynomial {
                degree: 16,
                // p(x) = x^16 + x^5 + x^3 + x^2 + 1
                coeffs: vec![
                    true, false, false, false, false, false, false, false, false, false, true,
                    false, true, true, false, true,
                ],
            };
        }
        32 => {
            return Polynomial {
                degree: 32,
                // p(x) = x^32 + x^22 + x^2 + x + 1
                coeffs: vec![
                    true, false, false, false, false, false, false, false, false, false, true,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, true, true, true,
                ],
            };
        }
        64 => {
            return Polynomial {
                degree: 64,
                // p(x) = x^64 + x^4 + x^3 + x + 1
                coeffs: vec![
                    true, // x^64
                    false, false, false, false, false, false, false, false,
                    false, // x^63..x^55
                    false, false, false, false, false, false, false, false,
                    false, // x^54..x^46
                    false, false, false, false, false, false, false, false,
                    false, // x^45..x^37
                    false, false, false, false, false, false, false, false,
                    false, // x^36..x^28
                    false, false, false, false, false, false, false, false,
                    false, // x^27..x^19
                    false, false, false, false, false, false, false, false, // x^18..x^11
                    false, false, // x^10, x^9
                    false, // x^8
                    false, // x^7
                    false, // x^6
                    false, // x^5
                    true,  // x^4
                    true,  // x^3
                    false, // x^2
                    true,  // x^1
                    true,  // x^0 (the constant term)
                ],
            };
        }
        _ => return Polynomial::zero(),
    }
}