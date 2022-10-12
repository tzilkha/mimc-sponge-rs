mod constants;
extern crate ff;
extern crate rand;
use ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
pub struct Fr(FrRepr);

// String -> Fr
pub fn str_to_fr(s: &str) -> Fr {
    todo!();
}

// Load constants
pub fn load_constants() -> Vec<Fr> {
    let c_str = constants::constants();
    let mut c: Vec<Fr> = Vec::new();
    for i in 0..c_str.len() {
        let b: Fr = str_to_fr(c_str[i]);
        c.push(b);
    }
    return c;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
