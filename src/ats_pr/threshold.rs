use crate::ats_pr::bls::ECPoint;
use crate::ats_pr::bls::ECScalar;
use crate::ats_pr::bls::KeyPairG2;
use crate::ats_pr::bls::FE2;
use crate::ats_pr::bls::GE2;
use crate::ats_pr::lagrange::lagrange_interpolate_f0_X;
use crate::ats_pr::lagrange::lagrange_interpolate_f0_x;

#[derive(Debug)]
pub struct ThresholdKeyPairs {
    pub keys: Vec<KeyPairG2>,
    n: usize,
    t: usize,
}

impl ThresholdKeyPairs {
    pub fn new(_n: usize, _t: usize) -> Self {
        Self {
            keys: vec![KeyPairG2::new(); _n],
            n: _n,
            t: _t,
        }
    }

    fn get_quorum_keys(&self, quorum: &Vec<usize>) -> Vec<&KeyPairG2> {
        let mut q: Vec<&KeyPairG2> = Vec::new();
        for idx in quorum {
            if *idx >= self.keys.len() {
                panic!("Quorum indices included party outside of available keys");
            }
            q.push(&self.keys[*idx]);
        }
        q
    }

    // [TODO] Convert functions below for generics that accept GE2 or FE2
    pub fn get_X(&self, quorum: &Vec<usize>) -> Vec<GE2> {
        self.get_quorum_keys(quorum)
            .into_iter()
            .map(|key: &KeyPairG2| key.X)
            .collect()
    }

    pub fn get_x(&self, quorum: &Vec<usize>) -> Vec<FE2> {
        self.get_quorum_keys(quorum)
            .into_iter()
            .map(|key: &KeyPairG2| key.x)
            .collect()
    }

    pub fn quorum_X(&self, quorum: &Vec<usize>) -> GE2 {
        lagrange_interpolate_f0_X(
            &quorum
                .into_iter()
                .map(|idx: &usize| idx + 1)
                .zip(self.get_X(&quorum).into_iter())
                .collect()
        )
    }

    pub fn quorum_x(&self, quorum: &Vec<usize>) -> FE2 {
        lagrange_interpolate_f0_x(
            &quorum
                .into_iter()
                .map(|idx: &usize| idx + 1)
                .zip(self.get_x(&quorum).into_iter())
                .collect()
        )
    }
}
