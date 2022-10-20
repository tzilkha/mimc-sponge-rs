#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::too_many_arguments)]
mod constants;

use constants::C_STR;
use ff::{self, *};
use once_cell::sync::Lazy;

const DEFAULT_CONSTS_LEN: usize = constants::C_STR.len();
static DEFAULT_CONSTS: Lazy<[Fr; DEFAULT_CONSTS_LEN]> = Lazy::new(|| {
    C_STR
        .iter()
        .map(|s| Fr::from_str(s).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
});

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
pub struct Fr(FrRepr);

pub struct MimcSponge {
    constants: [Fr; DEFAULT_CONSTS_LEN],
}

impl Default for MimcSponge {
    fn default() -> Self {
        Self {
            constants: *DEFAULT_CONSTS,
        }
    }
}

impl MimcSponge {
    fn hash(&self, mut xl: Fr, mut xr: Fr, k: Fr) -> (Fr, Fr) {
        let mut t;
        let mut xr_tmp;
        let last_index = self.constants.len() - 1;

        for (i, c) in self.constants.iter().enumerate() {
            t = Fr::zero();

            t.add_assign(&xl);
            t.add_assign(&k);

            if i > 0 {
                t.add_assign(c);
            }

            t = t.pow([5u64]);

            xr_tmp = xr;
            xr_tmp.add_assign(&t);

            if i < last_index {
                xr = xl;
                xl = xr_tmp;
            } else {
                xr = xr_tmp
            }
        }

        (xl, xr)
    }

    /// Takes &slice of Fr elements, key and num_outputs
    pub fn multi_hash(&self, arr: &[Fr], key: Fr, num_outputs: usize) -> Vec<Fr> {
        let mut r = Fr::zero();
        let mut c = Fr::zero();

        for elem in arr {
            r.add_assign(elem);
            let s: (Fr, Fr) = self.hash(r, c, key);
            (r, c) = s;
        }

        let mut out = Vec::with_capacity(num_outputs);
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
        assert_eq!(
            res[0].to_string(),
            "Fr(0x2bcea035a1251603f1ceaf73cd4ae89427c47075bb8e3a944039ff1e3d6d2a6f)"
        );
    }
}
