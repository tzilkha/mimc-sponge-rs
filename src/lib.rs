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
fn load_constants() -> [Fr; 220] {
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

pub struct MimcSponge {
    c: [Fr; 220],
}

impl Default for MimcSponge {
    fn default() -> Self {
        Self {
            c: load_constants(),
        }
    }
}

impl MimcSponge {
    fn hash(&self, _xL_in: Fr, _xR_in: Fr, _k: Fr) -> (Fr, Fr) {
        let mut t;
        let mut c;

        let mut xL = _xL_in;
        let mut xR = _xR_in;
        let mut xR_tmp;

        for i in 0..220 {
            c = self.c[i];
            t = Fr::zero();

            t.add_assign(&xL);
            t.add_assign(&_k);

            if i > 0 {
                t.add_assign(&c);
            }

            t = t.pow([5u64]);

            xR_tmp = xR;
            xR_tmp.add_assign(&t);

            if i < (219) {
                xR = xL;
                xL = xR_tmp;
            } else {
                xR = xR_tmp
            }
        }

        (xL, xR)
    }

    /// Takes &slice of Fr elements, key and num_outputs
    pub fn multi_hash(&self, arr: &[Fr], key: Fr, num_outputs: u64) -> Vec<Fr> {
        let mut r = Fr::zero();
        let mut c = Fr::zero();

        for elem in arr {
            r.add_assign(elem);
            let s: (Fr, Fr) = self.hash(r, c, key);
            (r, c) = s;
        }

        let mut out = Vec::new();
        out.push(r);

        for _ in 1..num_outputs {
            let s = self.hash(r, c, key);
            (r, c) = s;
            out.push(r);
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![Fr::from_str("1").unwrap(), Fr::from_str("2").unwrap()];

        let k = Fr::zero();

        let ms = MimcSponge::default();

        let res = ms.multi_hash(&arr, k, 1);

        let EXPECTED = "Fr(0x2bcea035a1251603f1ceaf73cd4ae89427c47075bb8e3a944039ff1e3d6d2a6f)";

        assert_eq!(res[0].to_string(), EXPECTED);
    }
}
