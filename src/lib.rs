#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::too_many_arguments)]

mod constants;

use ff::{self, *};

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
pub struct Fr(FrRepr);

// String -> Fr
pub fn str_to_fr(_s: &str) -> Fr {
    Fr::from_str(_s).unwrap()
}

// Load constants
// TODO make load_constants a const fn
pub fn load_constants() -> [Fr; 220] {
    let c_str = constants::C_STR;
    let mut c: [Fr; 220] = [Fr::zero(); 220];

    let mut i = 0;
    loop {
        if i < c_str.len() {
            let b: Fr = str_to_fr(c_str[i]);
            c[i] = b;
            i += 1;
        } else {
            return c;
        }
    }
}

pub fn hash(x_in: Fr, k: Fr) -> Fr {
    todo!();
}

pub fn multihash(arr: Vec<Fr>, key: Fr) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_constants();
    }
}
